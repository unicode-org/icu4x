// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crabbake::TokenStream;
use icu_provider::datagen::CrabbakeMarker;
use icu_provider::export::DataExporter;
use icu_provider::prelude::*;
use litemap::LiteMap;
use quote::{format_ident, quote};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

pub(crate) struct ConstExporter {
    data: Mutex<LiteMap<ResourceKey, LiteMap<ResourceOptions, String>>>,
    dir: PathBuf,
}

impl ConstExporter {
    pub fn new(dir: PathBuf) -> Result<Self, DataError> {
        std::fs::remove_dir_all(&dir)?;
        std::fs::create_dir(&dir)?;
        Ok(Self {
            data: Mutex::new(LiteMap::new()),
            dir,
        })
    }
}

impl DataExporter<CrabbakeMarker> for ConstExporter {
    fn put_payload(
        &self,
        key: ResourceKey,
        options: ResourceOptions,
        payload: DataPayload<CrabbakeMarker>,
    ) -> Result<(), DataError> {
        let mut map = self.data.lock().expect("poison");
        if !map.contains_key(&key) {
            map.insert(key, LiteMap::new());
        }
        // TokenStream isn't Send/Sync, so we need to stringify and later parse again
        map.get_mut(&key)
            .unwrap()
            .insert(options, payload.tokenize().to_string());
        Ok(())
    }

    fn close(&mut self) -> Result<(), DataError> {
        let data = {
            // move out of `data`
            let mut tmp = Mutex::new(LiteMap::new());
            std::mem::swap(&mut tmp, &mut self.data);
            tmp
        }
        .into_inner()
        .expect("poison");

        // One entry per key
        let mut field_declarations = Vec::new();
        let mut field_values = Vec::new();
        let mut data_provider_impls = Vec::new();

        // One entry per dependency
        let mut imports = std::collections::HashMap::new();

        for (key, data) in data.into_tuple_vec().into_iter() {
            let data = data
                .into_tuple_vec()
                .into_iter()
                .map(|(options, payload_bake_string)| {
                    // Bake the options as a string because we can compare them in that shape
                    let options_bake = options.to_string();
                    let payload_bake: TokenStream = payload_bake_string.parse().unwrap();
                    quote! { (#options_bake, #payload_bake)}
                })
                .collect::<Vec<_>>();

            let field_name =
                format_ident!("{}", key.get_path().to_string().replace(&['/', '@'], "_"));

            let (crate_name, crate_location) = crate::registry::get_crate(key);
            let crate_name_str = crate_name.to_string();
            let feature = quote! { feature = #crate_name_str };

            let (field_type, resource_marker) = crate::registry::get_struct_name(key);

            field_declarations.push((
                feature.clone(),
                quote! {
                    #field_name: &'static [(&'static str, #field_type)],
                },
            ));

            field_values.push((
                feature.clone(),
                quote! {
                    #field_name: &[#(#data),*],
                },
            ));

            data_provider_impls.push((feature.clone(), quote! {
                impl ResourceProvider<#resource_marker> for ConstProvider {
                    fn load_resource(&self, req: &DataRequest) -> Result<DataResponse<#resource_marker>, DataError> {
                        let index = self
                            .#field_name
                            .binary_search_by_key(&&*req.options.write_to_string(), |(k,_)| k)
                            .map_err(|_| DataErrorKind::MissingResourceOptions.into_error())?;
                        Ok(DataResponse {
                            metadata: DataResponseMetadata::default(),
                            payload: Some(DataPayload::from_owned(zerofrom::ZeroFrom::zero_from(
                                &self.#field_name[index].1,
                            ))),
                        })
                    }
                }
            }));

            imports.insert(
                (crate_name_str, crate_location),
                (feature, quote! { #crate_name::provider::* }),
            );
        }

        std::fs::copy(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("LICENSE"),
            self.dir.join("LICENSE"),
        )?;

        let mut cargo_toml = std::fs::File::create(self.dir.join("Cargo.toml"))?;

        // TODO: This assumes self.dir is in the icu directory
        let icu_root = PathBuf::from(
            "../".repeat(
                self.dir.canonicalize()?.components().count()
                    - PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                        .join("../..")
                        .canonicalize()?
                        .components()
                        .count(),
            ),
        );

        cargo_toml.write_all(
            b"\
            # GENERATED CRATE, DO NOT EDIT\n\
            [package]\n\
            name = \"",
        )?;
        cargo_toml.write_all(self.dir.file_name().unwrap().to_string_lossy().as_bytes())?;
        cargo_toml.write_all(
            b"\"\n\
            version = \"1.0.0\"\n\
            edition = \"2021\"\n\
            include = [\"data.rs\", \"Cargo.toml\"]\n\n\
            [lib]\n\
            path = \"data.rs\"\n\n\
            [dependencies]\n",
        )?;

        for (crate_name, crate_location) in [
            ("icu_provider", PathBuf::from("provider/core")),
            ("writeable", PathBuf::from("utils/writeable")),
        ]
        .into_iter()
        {
            cargo_toml.write_all(crate_name.as_bytes())?;
            cargo_toml.write_all(b" = { path = \"")?;
            cargo_toml.write_all(
                icu_root
                    .join(crate_location)
                    .as_os_str()
                    .to_string_lossy()
                    .as_bytes(),
            )?;
            cargo_toml.write_all(b"\"}\n")?;
        }

        for (crate_name, crate_location) in imports.keys() {
            cargo_toml.write_all(crate_name.as_bytes())?;
            cargo_toml.write_all(b" = { path = \"")?;
            cargo_toml.write_all(
                icu_root
                    .join(crate_location)
                    .as_os_str()
                    .to_string_lossy()
                    .as_bytes(),
            )?;
            cargo_toml.write_all(b"\", optional = true }\n")?;
        }

        let (fd_features, fd_tokens): (Vec<_>, Vec<_>) = field_declarations.into_iter().unzip();
        let (fv_features, fv_tokens): (Vec<_>, Vec<_>) = field_values.into_iter().unzip();
        let (dp_features, dp_tokens): (Vec<_>, Vec<_>) = data_provider_impls.into_iter().unzip();
        let (i_features, i_tokens): (Vec<_>, Vec<_>) = imports.into_values().unzip();

        let data = quote! {
            #![no_std]

            extern crate alloc;
            #[allow(unused_imports)]
            use writeable::Writeable;
            #[allow(unused_imports)]
            use icu_provider::prelude::*;

            #(
                #[cfg(#i_features)]
                use #i_tokens;
            )*

            pub struct ConstProvider {
                #(
                    #[cfg(#fd_features)]
                    #fd_tokens
                )*
            }

            #(
                #[cfg(#dp_features)]
                #dp_tokens
            )*

            pub const PROVIDER: ConstProvider = ConstProvider {
                #(
                    #[cfg(#fv_features)]
                    #fv_tokens
                )*
            };
        }
        .to_string();

        let data_rs = self.dir.join("data.rs");

        std::fs::File::create(&data_rs)?.write_all(b"// GENERATED CRATE. DO NOT EDIT\n\n")?;

        // Run rustfmt if we can. The raw TokenStream::to_string is unreadable.
        if let Ok(mut child) = std::process::Command::new("rustfmt")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::from(
                std::fs::OpenOptions::new().append(true).open(&data_rs)?,
            ))
            .spawn()
        {
            child.stdin.take().unwrap().write_all(data.as_bytes())?;
            child.wait()?;
        } else {
            std::fs::OpenOptions::new()
                .append(true)
                .open(&data_rs)?
                .write_all(data.as_bytes())?;
        }
        Ok(())
    }
}

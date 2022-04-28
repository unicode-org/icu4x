// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crabbake::{CrateEnv, TokenStream};
use icu_provider::datagen::CrabbakeMarker;
use icu_provider::export::DataExporter;
use icu_provider::prelude::*;
use litemap::LiteMap;
use quote::{format_ident, quote};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

macro_rules! move_out {
    ($field:expr) => {{
        let mut tmp = Default::default();
        core::mem::swap(&mut tmp, &mut $field);
        tmp
    }};
}

#[allow(clippy::type_complexity)]
pub(crate) struct ConstExporter {
    data: Mutex<LiteMap<ResourceKey, (String, LiteMap<String, String>)>>,
    dependencies: CrateEnv,
    mod_directory: PathBuf,
    pretty: bool,
}

impl ConstExporter {
    pub fn new(mod_directory: PathBuf, pretty: bool) -> Self {
        let _ = std::fs::remove_dir_all(&mod_directory);
        Self {
            data: Mutex::new(LiteMap::new()),
            dependencies: Default::default(),
            mod_directory,
            pretty,
        }
    }

    fn write_to_file(&self, mut file: File, data: TokenStream) -> Result<(), DataError> {
        file.write_all(b"// GENERATED MODULE. DO NOT EDIT\n\n")?;

        if self.pretty {
            let mut child = std::process::Command::new("rustfmt")
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::from(file))
                .spawn()
                .unwrap();
            child
                .stdin
                .take()
                .unwrap()
                .write_all(data.to_string().as_bytes())?;
            child.wait()?;
        } else {
            file.write_all(data.to_string().as_bytes())?;
        }
        Ok(())
    }
}

impl DataExporter<CrabbakeMarker> for ConstExporter {
    fn put_payload(
        &self,
        key: ResourceKey,
        options: ResourceOptions,
        payload: DataPayload<CrabbakeMarker>,
    ) -> Result<(), DataError> {
        // TokenStream isn't Send/Sync, so we need to stringify and later parse again
        let (payload, marker_type) = payload.tokenize(&self.dependencies);
        let mut map = self.data.lock().expect("poison");
        if !map.contains_key(&key) {
            map.insert(key, (marker_type.to_string(), LiteMap::new()));
        }
        let (existing_marker, data) = map.get_mut(&key).unwrap();
        debug_assert_eq!(&marker_type.to_string(), existing_marker);
        // We bake the options as a string because we can compare them in that shape
        data.insert(options.to_string(), payload.to_string());
        Ok(())
    }

    fn close(&mut self) -> Result<(), DataError> {
        let raw_values = move_out!(self.data).into_inner().expect("poison");

        let mut field_names = Vec::new();
        let mut resource_markers = Vec::new();
        let mut values = Vec::new();

        for (key, (marker, raw)) in raw_values.into_tuple_vec().into_iter() {
            values.push(
                raw.into_tuple_vec()
                    .into_iter()
                    .map(|(options_bake, payload_bake_string)| {
                        let payload_bake: TokenStream = payload_bake_string.parse().unwrap();
                        // Ref the payload so that LLVM can deduplicate them.
                        quote! { (#options_bake, &#payload_bake)}
                    })
                    .collect::<Vec<_>>(),
            );
            field_names.push(format_ident!(
                "{}",
                key.get_path().to_string().replace(&['/', '@', '='], "_")
            ));
            resource_markers.push(marker.parse::<TokenStream>().unwrap());
        }

        std::fs::create_dir(&self.mod_directory)?;

        self.dependencies.insert("icu_provider");
        self.dependencies.insert("writeable"); // TODO remove .to_string()s and compare ResourceOptions directly
        let mut deps = move_out!(self.dependencies).into_iter().collect::<Vec<_>>();
        deps.sort_unstable();
        log::info!("The generated module requires the following crates:");
        for crate_name in deps {
            log::info!("{}", crate_name);
        }

        self.write_to_file(File::create(self.mod_directory.join("mod.rs"))?, quote! {
            use writeable::Writeable;
            use icu_provider::prelude::*;

            pub struct StaticDataProvider {
                #(
                    #field_names: &'static [(&'static str, &'static <#resource_markers as DataMarker>::Yokeable)],
                )*
            }

            #(
                mod #field_names;
            )*

            pub static PROVIDER: &StaticDataProvider = &StaticDataProvider {
                #(
                    #field_names: #field_names::VALUES,
                )*
            };

            #(
                impl ResourceProvider<#resource_markers> for &'static StaticDataProvider {
                    fn load_resource(&self, req: &DataRequest) -> Result<DataResponse<#resource_markers>, DataError> {
                        let index = self
                            .#field_names
                            .binary_search_by_key(&&*req.options.write_to_string(), |(k,_)| k)
                            .map_err(|_| DataErrorKind::MissingResourceOptions.into_error())?;
                        Ok(DataResponse {
                            metadata: DataResponseMetadata::default(),
                            payload: Some(DataPayload::from_owned(zerofrom::ZeroFrom::zero_from(
                                self.#field_names[index].1,
                            ))),
                        })
                    }
                }
            )*
        })?;

        for ((field_name, resource_marker), values) in field_names
            .iter()
            .zip(resource_markers.iter())
            .zip(values.iter())
        {
            self.write_to_file(
                File::create(self.mod_directory.join(field_name.to_string()).with_extension("rs"))?, 
                quote! {
                    pub static VALUES: &[(&str, &<#resource_marker as ::icu_provider::DataMarker>::Yokeable)] = &[#(#values),*];
                }
            )?;
        }
        Ok(())
    }
}

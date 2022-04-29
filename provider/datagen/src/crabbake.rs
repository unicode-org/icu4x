// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crabbake::{quote, CrateEnv, TokenStream};
use icu_provider::datagen::CrabbakeMarker;
use icu_provider::export::DataExporter;
use icu_provider::prelude::*;
use litemap::LiteMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;

macro_rules! move_out {
    ($field:expr) => {{
        let mut tmp = Default::default();
        core::mem::swap(&mut tmp, &mut $field);
        tmp
    }};
}

// TokenStream isn't Send/Sync
type SyncTokenStream = String;

#[allow(clippy::type_complexity)]
pub(crate) struct ConstExporter {
    data: Mutex<LiteMap<ResourceKey, (SyncTokenStream, LiteMap<ResourceOptions, SyncTokenStream>)>>,
    fields: Mutex<Vec<(String, SyncTokenStream)>>,
    dependencies: CrateEnv,
    mod_directory: PathBuf,
    pretty: bool,
}

impl ConstExporter {
    pub fn new(mod_directory: PathBuf, pretty: bool) -> Self {
        let _ = std::fs::remove_dir_all(&mod_directory);
        Self {
            data: Mutex::new(LiteMap::new()),
            fields: Mutex::new(Vec::new()),
            dependencies: Default::default(),
            mod_directory,
            pretty,
        }
    }

    fn write_to_file(&self, path: &Path, data: TokenStream) -> Result<(), DataError> {
        std::fs::create_dir_all(&path.parent().unwrap())?;
        let mut file = File::create(path)?;

        file.write_all(
            b"// GENERATED MODULE. DO NOT EDIT\n\n\
              use ::icu_provider::prelude::*;\n\n",
        )?;

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
        let (payload, marker_type) = payload.tokenize(&self.dependencies);
        let mut map = self.data.lock().expect("poison");
        if !map.contains_key(&key) {
            map.insert(key, (marker_type.to_string(), LiteMap::new()));
        }
        map.get_mut(&key)
            .unwrap()
            .1
            .insert(options, payload.to_string());
        Ok(())
    }

    fn flush(&self, key: ResourceKey) -> Result<(), DataError> {
        if let Some((marker, raw)) = self.data.lock().expect("poison").remove(&key) {
            let baked = raw
                .into_tuple_vec()
                .into_iter()
                .map(|(options, payload_bake_string)| {
                    // We bake the options as a string because we can compare them in that shape
                    let options_bake: TokenStream = format!(r#""{}""#, options).parse().unwrap();
                    let payload_bake: TokenStream = payload_bake_string.parse().unwrap();
                    // We bake references to the data structs so that the compiler can deduplicate
                    // them.
                    quote! { (#options_bake, &#payload_bake) }
                });

            let ident = key.get_path().to_string().replace(&['/', '@', '='], "_");
            let marker = marker.parse::<TokenStream>().unwrap();

            self.write_to_file(&self.mod_directory.join(&ident).with_extension("rs"), 
                quote! {
                    pub static VALUES: &[(&str, &<#marker as DataMarker>::Yokeable)] = &[#(#baked),*];
                }
            )?;
            self.fields
                .lock()
                .expect("poison")
                .push((ident, marker.to_string()));
        }
        Ok(())
    }

    fn close(&mut self) -> Result<(), DataError> {
        self.dependencies.insert("icu_provider");
        let mut deps = move_out!(self.dependencies).into_iter().collect::<Vec<_>>();
        deps.sort_unstable();
        log::info!("The generated module requires the following crates:");
        for crate_name in deps {
            log::info!("{}", crate_name);
        }

        let mut fields = self.fields.lock().expect("poison");
        fields.sort_unstable();
        let (idents, markers): (Vec<TokenStream>, Vec<TokenStream>) = fields
            .iter()
            .map(|(ident, marker)| (ident.parse().unwrap(), marker.parse().unwrap()))
            .unzip();

        self.write_to_file(&self.mod_directory.join("mod.rs"), quote! {
            pub struct StaticDataProvider {
                #(
                    #idents: &'static [(&'static str, &'static <#markers as DataMarker>::Yokeable)],
                )*
            }

            #(
                mod #idents;
            )*

            pub static PROVIDER: &StaticDataProvider = &StaticDataProvider {
                #(
                    #idents: #idents::VALUES,
                )*
            };

            macro_rules! provider_impl {
                ($ marker : ty , $ field_name : ident) => {
                    impl ResourceProvider<$marker> for &'static StaticDataProvider {
                        fn load_resource(
                            &self,
                            req: &DataRequest,
                        ) -> Result<DataResponse<$marker>, DataError> {
                            let value = self
                                .$field_name
                                .binary_search_by(|(k, _)| req.options.cmp_bytes(k.as_bytes()).reverse())
                                .map(|i| self.$field_name.get(i).unwrap().1)
                                .map_err(|_| {
                                    DataErrorKind::MissingResourceOptions.with_req(<$marker>::KEY, req)
                                })?;
                            Ok(DataResponse {
                                metadata: DataResponseMetadata::default(),
                                payload: Some(DataPayload::from_owned(zerofrom::ZeroFrom::zero_from(value))),
                            })
                        }
                    }
                };
            }

            #(
                provider_impl!(#markers, #idents);
            )*
        })?;

        Ok(())
    }
}

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

        file.write_all(b"// GENERATED MODULE. DO NOT EDIT\n")?;

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
            let ident = key.get_path().to_string().replace(&['/', '@', '='], "_");
            let marker = marker.parse::<TokenStream>().unwrap();

            let mut sorted = raw.into_tuple_vec();
            // Sort by payload bake so we can deduplicate
            sorted.sort_unstable_by(|(_, a), (_, b)| a.cmp(b));

            let mut statics = Vec::new();
            let mut all_options = Vec::new();

            for (payload_bake_string, group) in
                &itertools::Itertools::group_by(sorted.into_iter(), |(_, payload_bake_string)| {
                    payload_bake_string.clone()
                })
            {
                let options = group
                    .map(|(options, _)| options.to_string())
                    .collect::<Vec<_>>();
                let ident_string = options
                    .iter()
                    .map(|options| {
                        let mut string = options.replace('-', "_");
                        string.make_ascii_uppercase();
                        string
                    })
                    .reduce(|mut a, b| {
                        a.push('_');
                        a.push_str(&b);
                        a
                    })
                    .unwrap();
                all_options.extend(options.into_iter().map(|o| (o, ident_string.clone())));
                statics.push((ident_string, payload_bake_string));
            }

            // Not necessary for functionality, but prettier
            statics.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

            let statics = statics
                .into_iter()
                .map(|(ident_string, payload_bake_string)| {
                    let ident = ident_string.parse::<TokenStream>().unwrap();
                    let payload_bake = payload_bake_string.parse::<TokenStream>().unwrap();
                    quote! { static #ident: super::DataStruct<#marker> = &#payload_bake; }
                })
                .collect::<Vec<_>>();

            all_options.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
            let all_options = all_options
                .into_iter()
                .map(|(options, ident_string)| {
                    let ident = ident_string.parse::<TokenStream>().unwrap();
                    quote! { (#options, #ident) }
                })
                .collect::<Vec<_>>();

            self.write_to_file(
                &self.mod_directory.join(&ident).with_extension("rs"),
                quote! {
                    pub static VALUES: super::Data<#marker> = &[#(#all_options),*];
                    #(#statics)*
                },
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
            type DataStruct<M> = &'static <M as ::icu_provider::DataMarker>::Yokeable;
            type Options = &'static str;
            type Data<M> = &'static [(Options, DataStruct<M>)];

            pub struct BakedDataProvider {
                #(
                    #idents: Data<#markers>,
                )*
            }

            #(
                mod #idents;
            )*

            pub static PROVIDER: &BakedDataProvider = &BakedDataProvider {
                #(
                    #idents: #idents::VALUES,
                )*
            };

            use ::icu_provider::prelude::*;
            macro_rules! provider_impl {
                ($ marker : ty , $ field_name : ident) => {
                    impl ResourceProvider<$marker> for &'static BakedDataProvider {
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

// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crabbake::{quote, CrateEnv, TokenStream};
use icu_provider::datagen::CrabbakeMarker;
use icu_provider::export::DataExporter;
use icu_provider::prelude::*;
use itertools::Itertools;
use litemap::LiteMap;
use std::collections::HashMap;
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

// TokenStream isn't Send/Sync
type SyncTokenStream = String;
// syn::Path also isn't Send/Sync
type Path = Vec<String>;

#[allow(clippy::type_complexity)]
pub(crate) struct ConstExporter {
    // Input arguments
    mod_directory: PathBuf,
    pretty: bool,
    insert_feature_gates: bool,
    // Temporary storage for put_payload: key -> (marker path, options -> bake)
    data: Mutex<LiteMap<ResourceKey, (SyncTokenStream, LiteMap<ResourceOptions, SyncTokenStream>)>>,
    // (field, module path, marker path) after flushing each key
    per_key_data: Mutex<Vec<(String, Path, Path)>>,
    // All mod.rs files in the module tree. Because generation is parallel,
    // this will be non-deterministic and have to be sorted later.
    mod_files: Mutex<HashMap<PathBuf, Vec<String>>>,
    // List of dependencies used by baking.
    dependencies: CrateEnv,
}

impl ConstExporter {
    pub fn new(mod_directory: PathBuf, pretty: bool, insert_feature_gates: bool) -> Self {
        let _ = std::fs::remove_dir_all(&mod_directory);
        Self {
            mod_directory,
            pretty,
            insert_feature_gates,
            data: Default::default(),
            per_key_data: Default::default(),
            mod_files: Default::default(),
            dependencies: Default::default(),
        }
    }

    fn write_to_file<P: AsRef<std::path::Path>>(
        &self,
        path: P,
        data: TokenStream,
    ) -> Result<(), DataError> {
        let path = self.mod_directory.join(path).with_extension("rs");
        std::fs::create_dir_all(&path.parent().unwrap())?;

        {
            let mut file = line_ending_file::BufWriterWithLineEndingFix::new(File::create(&path)?);
            writeln!(file, "// GENERATED MODULE. DO NOT EDIT")?;
            writeln!(file, "{}", data)?;
        }

        if self.pretty {
            std::process::Command::new("rustfmt")
                .arg(path)
                .spawn()
                .unwrap()
                .wait()?;
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
            let marker = syn::parse_str::<syn::Path>(&marker).unwrap();

            let mut sorted = raw.into_tuple_vec();
            // Sort by payload bake so we can deduplicate.
            sorted.sort_by(|(_, a), (_, b)| a.cmp(b));

            let mut statics = Vec::new();
            let mut all_options = Vec::new();

            for (payload_bake_string, group) in &sorted
                .into_iter()
                .group_by(|(_, payload_bake_string)| payload_bake_string.clone())
            {
                // These are sorted because we stably sorted earlier.
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
                    quote! { static #ident: DataStruct<#marker> = &#payload_bake; }
                });

            all_options.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
            let all_options = all_options.into_iter().map(|(options, ident_string)| {
                let ident = ident_string.parse::<TokenStream>().unwrap();
                quote! { (#options, #ident) }
            });

            let module_path = syn::parse_str::<syn::Path>(
                &key.get_path().replace(&['=', '@'], "_").replace('/', "::"),
            )
            .map_err(|_| {
                DataError::custom("Key component is not a valid Rust identifier").with_key(key)
            })?;

            // Intialise intermediate "mod.rs"s.
            let mut path = PathBuf::new();
            let depth = module_path.segments.len() - 1;
            for i in 1..=depth {
                path = path.join(module_path.segments[i - 1].ident.to_string());

                let mod_path = path.join("mod.rs");
                let mut map = self.mod_files.lock().expect("poison");
                if !map.contains_key(&mod_path) {
                    map.insert(mod_path.clone(), Vec::new());
                }
                map.get_mut(&mod_path)
                    .unwrap()
                    .push(module_path.segments[i].ident.to_string());
            }

            path = path
                .join(module_path.segments[depth].ident.to_string())
                .with_extension("rs");

            let supers = std::iter::repeat("super::")
                .take(depth)
                .collect::<String>()
                .parse::<TokenStream>()
                .unwrap();

            self.write_to_file(
                &path,
                quote! {
                    use #supers Data;
                    use #supers DataStruct;
                    pub static VALUES: Data<#marker> = &[#(#all_options),*];
                    #(#statics)*
                },
            )?;
            self.per_key_data.lock().expect("poison").push((
                key.get_path().replace(&['=', '@', '/'], "_"),
                module_path
                    .segments
                    .into_iter()
                    .map(|i| i.ident.to_string())
                    .collect(),
                marker
                    .segments
                    .into_iter()
                    .map(|i| i.ident.to_string())
                    .collect(),
            ));
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

        for (path, mods) in self.mod_files.lock().expect("poison").drain() {
            let mods = mods.into_iter().map(|p| p.parse::<TokenStream>().unwrap());
            self.write_to_file(
                &path,
                quote! {
                    #(
                        mod #mods;
                    )*
                },
            )?;
        }

        let mut data = self.per_key_data.lock().expect("poison");
        // data is populated concurrently, so this is required for stability.
        data.sort_unstable();

        // One per key
        let mut module_paths = Vec::new();
        let mut fields = Vec::new();
        let mut markers = Vec::new();
        let mut feature_gates = Vec::new();

        // One per submodule
        let mut mods = Vec::new();

        for (field, module_path, marker) in data.iter() {
            let module_path = module_path
                .into_iter()
                .map(|p| syn::parse_str::<syn::Ident>(p).unwrap())
                .collect::<Vec<_>>();
            let marker = marker
                .into_iter()
                .map(|m| syn::parse_str::<syn::Ident>(m).unwrap())
                .collect::<Vec<_>>();
            module_paths.push(quote! {#(#module_path)::*});
            fields.push(syn::parse_str::<syn::Ident>(field).unwrap());
            markers.push(quote! {#(::#marker)*});
            feature_gates.push(if self.insert_feature_gates {
                let feature = marker[0].to_string();
                quote! { #[cfg(feature = #feature)] }
            } else {
                quote!()
            });
            mods.push(module_path.into_iter().next().unwrap());
        }

        let mods = mods
            .into_iter()
            .zip(feature_gates.iter())
            .unique_by(|(modd, _)| modd.to_string())
            .map(|(modd, feature)| {
                quote! { #feature mod #modd; }
            });

        self.write_to_file("mod", quote! {
            type DataStruct<M> = &'static <M as ::icu_provider::DataMarker>::Yokeable;
            type Options = &'static str;
            type Data<M> = &'static [(Options, DataStruct<M>)];

            pub struct BakedDataProvider {
                #(
                    #feature_gates
                    #fields: Data<#markers>,
                )*
            }

            #(#mods)*

            pub static PROVIDER: &BakedDataProvider = &BakedDataProvider {
                #(
                    #feature_gates
                    #fields: #module_paths::VALUES,
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
                #feature_gates
                provider_impl!(#markers, #fields);
            )*
        })?;

        Ok(())
    }
}

// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crabbake::{quote, CrateEnv, TokenStream};
use icu_provider::datagen::*;
use icu_provider::prelude::*;
use itertools::Itertools;
use litemap::LiteMap;
use rayon::prelude::*;
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

#[allow(clippy::type_complexity)]
pub(crate) struct ConstExporter {
    // Input arguments
    mod_directory: PathBuf,
    pretty: bool,
    insert_feature_gates: bool,
    // Temporary storage for put_payload: key -> (marker path, options -> bake)
    data: Mutex<LiteMap<ResourceKey, (SyncTokenStream, LiteMap<ResourceOptions, SyncTokenStream>)>>,
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
            mod_files: Default::default(),
            dependencies: Default::default(),
        }
    }

    fn write_to_file<P: AsRef<std::path::Path>>(
        &self,
        path: P,
        data: TokenStream,
    ) -> Result<(), DataError> {
        let path = self.mod_directory.join(path).with_extension("raw");
        std::fs::create_dir_all(&path.parent().unwrap())?;

        {
            let mut file = crlify::BufWriterWithLineEndingFix::new(File::create(&path)?);
            writeln!(file, "// @generated")?;
            writeln!(file, "{}", data)?;
        }

        if self.pretty {
            std::process::Command::new("rustfmt")
                // When called on a file, rustfmt also formats all submodules.
                // Because we might have massive submodules that are already
                // formatted, we don't want this. Currently the only way to
                // disable this behaviour seems to be to use stdin/stdout.
                .stdin(std::process::Stdio::from(File::open(&path)?))
                .stdout(std::process::Stdio::from(File::create(
                    path.with_extension("rs"),
                )?))
                // The default, "auto", is meant to detect the existing line endings and preserve them.
                // However, this seems to be broken and generates Unix line endings on Windows, which
                // introduces Git diffs when regenerating.
                .args(&[
                    "--config",
                    "newline_style=native,format_generated_files=true,normalize_doc_attributes=true",
                ])
                .spawn()
                .unwrap()
                .wait()?;
            std::fs::remove_file(&path)?;
        } else {
            std::fs::rename(&path, path.with_extension("rs"))?
        }
        Ok(())
    }
}

impl DataExporter for ConstExporter {
    fn put_payload(
        &self,
        key: ResourceKey,
        options: &ResourceOptions,
        payload: &DataPayload<ExportMarker>,
    ) -> Result<(), DataError> {
        let (payload, marker_type) = payload.tokenize(&self.dependencies);
        let payload_string = payload.to_string();
        let mut map = self.data.lock().expect("poison");
        if !map.contains_key(&key) {
            map.insert(key, (marker_type.to_string(), LiteMap::new()));
        }
        map.get_mut(&key)
            .unwrap()
            .1
            .insert(options.clone(), payload_string);
        Ok(())
    }

    fn flush(&self, key: ResourceKey) -> Result<(), DataError> {
        let tmp = self.data.lock().expect("poison").remove(&key);
        if let Some((marker, raw)) = tmp {
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
                        // Cap identifier length at around 35
                        if a.len() < 35 {
                            a.push('_');
                            a.push_str(&b);
                        }
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
                    quote! { static #ident: DataStruct = &#payload_bake; }
                });

            all_options.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
            let all_options = all_options.into_iter().map(|(options, ident_string)| {
                let ident = ident_string.parse::<TokenStream>().unwrap();
                quote! { (#options, #ident) }
            });

            // Replace non-ident-allowed tokens. This can still fail if a segment starts with
            // a token that is not allowed in an initial position.
            let module_path = syn::parse_str::<syn::Path>(
                &key.get_path()
                    .to_ascii_lowercase()
                    .replace('@', "_v")
                    .replace('/', "::"),
            )
            .map_err(|_| {
                DataError::custom("Key component is not a valid Rust identifier").with_key(key)
            })?;

            let marker = syn::parse_str::<syn::Path>(&marker).unwrap();

            let feature = if self.insert_feature_gates {
                let feature = marker.segments.iter().next().unwrap().ident.to_string();
                quote! { #![cfg(feature = #feature)] }
            } else {
                quote!()
            };

            let mut path = PathBuf::new();
            let mut supers = quote!();
            for level in module_path.segments {
                let mut map = self.mod_files.lock().expect("poison");
                if !map.contains_key(&path) {
                    map.insert(path.clone(), Vec::new());
                }
                map.get_mut(&path).unwrap().push(level.ident.to_string());
                path = path.join(level.ident.to_string());
                supers = quote! { super:: #supers };
            }

            let struct_type =
                if key == icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker::KEY {
                    quote! {
                        &'static [(
                            &'static [::icu_datetime::fields::Field],
                            ::icu_datetime::pattern::runtime::PatternPlurals<'static>
                        )]
                    }
                } else {
                    quote! {
                        &'static <#marker as DataMarker>::Yokeable
                    }
                };

            self.write_to_file(
                &path,
                quote! {
                    #feature

                    use ::icu_provider::prelude::*;

                    impl ResourceProvider<#marker> for #supers BakedDataProvider {
                        fn load_resource(
                            &self,
                            req: &DataRequest,
                        ) -> Result<DataResponse<#marker>, DataError> {
                            static VALUES: &[(&str, DataStruct)] = &[#(#all_options),*];
                            #[allow(clippy::unwrap_used)] // binary search Ok() is safe to index
                            let value = VALUES
                                .binary_search_by(|(k, _)| req.options.strict_cmp(k.as_bytes()).reverse())
                                .map(|i| VALUES.get(i).unwrap().1)
                                .map_err(|_| {
                                    DataErrorKind::MissingResourceOptions.with_req(<#marker>::KEY, req)
                                })?;
                            Ok(DataResponse {
                                metadata: DataResponseMetadata::default(),
                                payload: Some(DataPayload::from_owned(zerofrom::ZeroFrom::zero_from(value))),
                            })
                        }
                    }

                    type DataStruct = #struct_type;

                    #(#statics)*
                },
            ).map_err(|e| e.with_path_context(&path))?;
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

        move_out!(self.mod_files)
            .into_inner()
            .expect("poison")
            .into_par_iter()
            .try_for_each(|(path, mut mods)| {
                mods.sort();
                let mods = mods
                    .into_iter()
                    .dedup()
                    .map(|p| p.parse::<TokenStream>().unwrap());
                let maybe_decl = if path.as_os_str().is_empty() {
                    quote! {
                        /// This data provider was programmatically generated by [`icu_datagen`](
                        /// https://unicode-org.github.io/icu4x-docs/doc/icu_datagen/enum.Out.html#variant.Module).
                        #[non_exhaustive]
                        pub struct BakedDataProvider;
                    }
                } else {
                    quote!()
                };
                self.write_to_file(
                    &path.join("mod"),
                    quote! {
                        #maybe_decl
                        #(
                            mod #mods;
                        )*
                    },
                )
                .map_err(|e| e.with_path_context(&path.join("mod/*")))
            })?;

        Ok(())
    }
}

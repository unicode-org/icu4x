// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use databake::{quote, CrateEnv, TokenStream};
use icu_provider::datagen::*;
use icu_provider::prelude::*;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::BTreeSet;
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
pub(crate) struct BakedDataExporter {
    // Input arguments
    mod_directory: PathBuf,
    pretty: bool,
    insert_feature_gates: bool,
    use_separate_crates: bool,
    // Temporary storage for put_payload: key -> (bake -> [locale])
    data: Mutex<HashMap<DataKey, HashMap<SyncTokenStream, Vec<String>>>>,
    // All mod.rs files in the module tree. Because generation is parallel,
    // this will be non-deterministic and have to be sorted later.
    mod_files: Mutex<HashMap<PathBuf, Vec<String>>>,
    /// Triples of the data marker, the path to the DATA slice (if there's data), and the feature
    /// that includes it. This is populated by `flush` and consumed by `close` which writes the
    /// implementations.
    marker_data_feature: Mutex<Vec<(SyncTokenStream, Option<SyncTokenStream>, SyncTokenStream)>>,
    // List of dependencies used by baking.
    dependencies: CrateEnv,
}

impl BakedDataExporter {
    pub fn new(
        mod_directory: PathBuf,
        pretty: bool,
        insert_feature_gates: bool,
        use_separate_crates: bool,
    ) -> Self {
        let _ = std::fs::remove_dir_all(&mod_directory);
        Self {
            mod_directory,
            pretty,
            insert_feature_gates: insert_feature_gates && use_separate_crates,
            use_separate_crates,
            data: Default::default(),
            mod_files: Default::default(),
            marker_data_feature: Default::default(),
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
            if self.use_separate_crates {
                writeln!(file, "{}", data)?;
            } else {
                writeln!(
                    file,
                    "{}",
                    data.to_string()
                        .replace("icu_", "icu :: ")
                        .replace("icu :: provider", "icu_provider")
                        .replace("icu :: datagen", "icu_datagen")
                )?;
            }
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
                .args(&[
                    // The default, "auto", is meant to detect the existing line endings and preserve them.
                    // However, this seems to be broken and generates Unix line endings on Windows.
                    "--config=newline_style=native",
                    // false by default, which is nice because then cargo fmt won't touch these again
                    "--config=format_generated_files=true",
                    // quote! stringifies doc comments as attributes, which is not very readable
                    "--config=normalize_doc_attributes=true",
                    // Defaults to 2015, but we're outputting 2021
                    "--edition=2021",
                    // Rustfmt silently gives up if it cannot achieve the max width, which happens for the root mod.rs
                    if path.file_stem().and_then(std::ffi::OsStr::to_str) == Some("mod") {
                        "--config=max_width=150"
                    } else {
                        "--config=max_width=100"
                    },
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

impl DataExporter for BakedDataExporter {
    fn put_payload(
        &self,
        key: DataKey,
        locale: &DataLocale,
        payload: &DataPayload<ExportMarker>,
    ) -> Result<(), DataError> {
        self.dependencies.insert("litemap");
        let payload = payload.tokenize(&self.dependencies);
        self.data
            .lock()
            .expect("poison")
            .entry(key)
            .or_default()
            .entry(payload.to_string())
            .or_default()
            .push(locale.to_string());
        Ok(())
    }

    fn flush(&self, key: DataKey) -> Result<(), DataError> {
        let marker =
            syn::parse2::<syn::Path>(crate::registry::key_to_marker_bake(key, &self.dependencies))
                .unwrap();

        let feature = if self.insert_feature_gates {
            let feature = marker.segments.iter().next().unwrap().ident.to_string();
            #[allow(unused_mut)]
            let mut feature = if !feature.starts_with("icu_provider") {
                quote! { #![cfg(feature = #feature)] }
            } else {
                quote!()
            };
            #[cfg(feature = "experimental")]
            if key == icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker::KEY {
                feature = quote! { #![cfg(feature = "icu_datetime_experimental")] }
            }
            feature
        } else {
            quote!()
        };

        let data = if let Some(raw) = self.data.lock().expect("poison").remove(&key) {
            let mut statics = Vec::new();
            let mut all_locales = Vec::new();

            for (payload_bake_string, mut locales) in raw {
                locales.sort();
                let ident_string = locales
                    .iter()
                    .map(|locales| {
                        let mut string = locales.replace('-', "_");
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
                all_locales.extend(locales.into_iter().map(|l| (l, ident_string.clone())));
                statics.push((ident_string, payload_bake_string));
            }

            statics.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

            let statics = statics
                .into_iter()
                .map(|(ident_string, payload_bake_string)| {
                    let ident = ident_string.parse::<TokenStream>().unwrap();
                    let payload_bake = payload_bake_string.parse::<TokenStream>().unwrap();
                    quote! { static #ident: &DataStruct = &#payload_bake; }
                });

            all_locales.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
            let data = all_locales.into_iter().map(|(locale, ident_string)| {
                let ident = ident_string.parse::<TokenStream>().unwrap();
                quote! { (#locale, #ident) }
            });

            // Replace non-ident-allowed tokens. This can still fail if a segment starts with
            // a token that is not allowed in an initial position.
            let module_path = syn::parse_str::<syn::Path>(
                &key.path()
                    .to_ascii_lowercase()
                    .replace('@', "_v")
                    .replace('/', "::"),
            )
            .map_err(|_| {
                DataError::custom("Key component is not a valid Rust identifier").with_key(key)
            })?;

            #[allow(unused_mut)]
            let mut struct_type = quote! { <#marker as ::icu_provider::DataMarker>::Yokeable };

            #[cfg(feature = "experimental")]
            if key == icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker::KEY {
                struct_type = quote! {
                    [(
                        &'static [::icu_datetime::fields::Field],
                        ::icu_datetime::pattern::runtime::PatternPlurals<'static>
                    )]
                };
            }

            let mut path = PathBuf::new();
            for level in &module_path.segments {
                self.mod_files
                    .lock()
                    .expect("poison")
                    .entry(path.clone())
                    .or_default()
                    .push(level.ident.to_string());
                path = path.join(level.ident.to_string());
            }

            self.write_to_file(
                &path,
                quote! {
                    #feature

                    type DataStruct = #struct_type;

                    pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
                        litemap::LiteMap::from_sorted_store_unchecked(&[#(#data),*]);

                    #(#statics)*
                },
            )
            .map_err(|e| e.with_path_context(&path))?;

            Some(quote!(#module_path).to_string())
        } else {
            None
        };

        self.marker_data_feature.lock().expect("poison").push((
            quote!(#marker).to_string(),
            data,
            feature.to_string().replacen("# ! [", "# [", 1),
        ));
        Ok(())
    }

    fn close(&mut self) -> Result<(), DataError> {
        self.dependencies.insert("icu_provider");
        // TODO: make locale fallback cfg'ed
        self.dependencies.insert("icu_provider_adapters");
        let mut deps = move_out!(self.dependencies)
            .into_iter()
            .collect::<BTreeSet<_>>();
        if !self.use_separate_crates {
            deps.retain(|&krate| krate == "icu_provider" || !krate.starts_with("icu_"));
            deps.insert("icu");
        }
        log::info!("The generated module requires the following crates:");
        for crate_name in deps {
            log::info!("{}", crate_name);
        }

        let mut mod_files = move_out!(self.mod_files).into_inner().expect("poison");
        for (_, mods) in mod_files.iter_mut() {
            mods.sort();
        }

        let mods = mod_files
            .remove(&PathBuf::new())
            .unwrap_or_default()
            .into_iter()
            .dedup()
            .map(|p| p.parse::<TokenStream>().unwrap());

        let mut marker_data_feature = move_out!(self.marker_data_feature)
            .into_inner()
            .expect("poison");
        marker_data_feature.sort();
        let marker_data_feature_ident = marker_data_feature
            .into_iter()
            .map(|(marker_str, data_str, feature_str)| {
                (
                    marker_str.parse::<TokenStream>().unwrap(),
                    data_str.map(|d| d.parse::<TokenStream>().unwrap()),
                    feature_str.parse::<TokenStream>().unwrap(),
                    marker_str
                        .split(' ')
                        .next_back()
                        .unwrap()
                        .to_ascii_uppercase()
                        .parse::<TokenStream>()
                        .unwrap(),
                )
            })
            .collect::<Vec<_>>();

        let resource_impls = marker_data_feature_ident.iter().map(|(marker, data, feature, _)| {
            if let Some(data) = data {
                quote! {
                    #feature
                    impl DataProvider<#marker> for BakedDataProvider {
                        fn load(
                            &self,
                            req: DataRequest,
                        ) -> Result<DataResponse<#marker>, DataError> {
                            Ok(DataResponse {
                                metadata: Default::default(),
                                payload: Some(DataPayload::from_owned(zerofrom::ZeroFrom::zero_from(
                                    *#data::DATA
                                        .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                                        .ok_or_else(|| DataErrorKind::MissingLocale.with_req(#marker::KEY, req))?
                                ))),
                            })
                        }
                    }
                }
            } else {
                quote! {
                    #feature
                    impl DataProvider<#marker> for BakedDataProvider {
                        fn load(
                            &self,
                            req: DataRequest,
                        ) -> Result<DataResponse<#marker>, DataError> {
                            Err(DataErrorKind::MissingLocale.with_req(#marker::KEY, req))
                        }
                    }
                }
            }
        });

        self.write_to_file(
            PathBuf::from("mod"),
            quote! {
                #(
                    mod #mods;
                )*

                /// This data provider was programmatically generated by [`icu_datagen`](
                /// https://unicode-org.github.io/icu4x-docs/doc/icu_datagen/enum.Out.html#variant.Module).
                #[non_exhaustive]
                pub struct BakedDataProvider;
                use ::icu_provider::prelude::*;

                #(#resource_impls)*
            },
        )
        .map_err(|e| e.with_path_context(&PathBuf::from("mod.rs")))?;

        let any_consts = marker_data_feature_ident
            .iter()
            .map(|(marker, _, feature, ident)| {
                quote! {
                    #feature
                    const #ident: ::icu_provider::DataKeyHash = #marker::KEY.hashed();
                }
            });

        let any_cases = marker_data_feature_ident
            .iter()
            .map(|(marker, data, feature, ident)| {
                if let Some(data) = data {
                    // TODO(#1678): Remove the special case
                    if marker.to_string()
                        == ":: icu_datetime :: provider :: calendar :: DateSkeletonPatternsV1Marker"
                    {
                        quote! {
                            #feature
                            #ident => {
                                #data::DATA
                                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                                    .copied()
                                    .map(zerofrom::ZeroFrom::zero_from)
                                    .map(DataPayload::<#marker>::from_owned)
                                    .map(DataPayload::wrap_into_any_payload)
                                    .ok_or(DataErrorKind::MissingLocale)
                            }
                        }
                    } else {
                        quote! {
                            #feature
                            #ident => {
                                #data::DATA
                                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                                    .copied()
                                    .map(AnyPayload::from_static_ref)
                                    .ok_or(DataErrorKind::MissingLocale)
                            }
                        }
                    }
                } else {
                    quote! {
                        #feature
                        #ident => Err(DataErrorKind::MissingLocale),
                    }
                }
            });

        self.write_to_file(
            PathBuf::from("any"),
            quote! {
                impl AnyProvider for BakedDataProvider {
                    fn load_any(&self, key: DataKey, req: DataRequest) -> Result<AnyResponse, DataError> {
                        #(#any_consts)*
                        #[allow(clippy::match_single_binding)]
                        match key.hashed() {
                            #(#any_cases)*
                            _ => Err(DataErrorKind::MissingDataKey),
                        }
                        .map_err(|e| e.with_req(key, req))
                        .map(|payload| AnyResponse {
                            payload: Some(payload),
                            metadata: Default::default(),
                        })
                    }
                }
            }
        )
        .map_err(|e| e.with_path_context(&PathBuf::from("any.rs")))?;

        mod_files.into_par_iter().try_for_each(|(path, mut mods)| {
            mods.sort();
            let mods = mods
                .into_iter()
                .dedup()
                .map(|p| p.parse::<TokenStream>().unwrap());
            self.write_to_file(
                &path.join("mod"),
                quote! {
                    #(
                        pub mod #mods;
                    )*
                },
            )
            .map_err(|e| e.with_path_context(&path.join("mod.rs")))
        })?;

        Ok(())
    }
}

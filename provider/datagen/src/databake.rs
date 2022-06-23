// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use databake::{quote, CrateEnv, TokenStream};
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
pub(crate) struct BakedDataExporter {
    // Input arguments
    mod_directory: PathBuf,
    pretty: bool,
    insert_feature_gates: bool,
    // Temporary storage for put_payload: key -> (marker path, options -> bake)
    data: Mutex<LiteMap<ResourceKey, (SyncTokenStream, LiteMap<ResourceOptions, SyncTokenStream>)>>,
    // All mod.rs files in the module tree. Because generation is parallel,
    // this will be non-deterministic and have to be sorted later.
    mod_files: Mutex<HashMap<PathBuf, Vec<String>>>,
    /// Triples of the ResourceMarker, the path to the DATA slice, and the feature that includes it.
    /// This is populated by `put_payload` and consumed by `flush` which writes the implementations.
    marker_data_feature: Mutex<Vec<(SyncTokenStream, SyncTokenStream, SyncTokenStream)>>,
    // List of dependencies used by baking.
    dependencies: CrateEnv,
}

impl BakedDataExporter {
    pub fn new(mod_directory: PathBuf, pretty: bool, insert_feature_gates: bool) -> Self {
        let _ = std::fs::remove_dir_all(&mod_directory);
        Self {
            mod_directory,
            pretty,
            insert_feature_gates,
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

impl DataExporter for BakedDataExporter {
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
            for level in &module_path.segments {
                let mut map = self.mod_files.lock().expect("poison");
                if !map.contains_key(&path) {
                    map.insert(path.clone(), Vec::new());
                }
                map.get_mut(&path).unwrap().push(level.ident.to_string());
                drop(map);
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
                        &'static <#marker as ::icu_provider::DataMarker>::Yokeable
                    }
                };

            self.write_to_file(
                &path,
                quote! {
                    #feature

                    type DataStruct = #struct_type;

                    pub static DATA: &[(&str, DataStruct)] = &[#(#all_options),*];

                    #(#statics)*
                },
            )
            .map_err(|e| e.with_path_context(&path))?;

            self.marker_data_feature.lock().expect("poison").push((
                quote!(#marker).to_string(),
                quote!(#module_path).to_string(),
                if self.insert_feature_gates {
                    let feature = marker.segments.iter().next().unwrap().ident.to_string();
                    quote! { #[cfg(feature = #feature)] }.to_string()
                } else {
                    String::new()
                },
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

        let mut mod_files = move_out!(self.mod_files).into_inner().expect("poison");
        for (_, mods) in mod_files.iter_mut() {
            mods.sort();
        }

        let mods = mod_files
            .remove(&PathBuf::new())
            .expect("root exists")
            .into_iter()
            .dedup()
            .map(|p| p.parse::<TokenStream>().unwrap());

        let marker_data_feature = move_out!(self.marker_data_feature)
            .into_inner()
            .expect("poison")
            .into_iter()
            .map(|(marker_str, data_str, feature_str)| {
                (
                    marker_str.parse::<TokenStream>().unwrap(),
                    data_str.parse::<TokenStream>().unwrap(),
                    feature_str.parse::<TokenStream>().unwrap(),
                )
            })
            .collect::<Vec<_>>();

        let any_cases = marker_data_feature.iter().map(|(marker, data, feature)| {
            if marker.to_string() == ":: icu_datetime :: provider :: calendar :: DateSkeletonPatternsV1Marker" {
                quote! {
                    #feature
                    <#marker as ::icu_provider::ResourceMarker>::KEY =>
                        ::icu_provider::AnyPayload::from_rc_payload::<#marker>(
                            alloc::rc::Rc::new(
                                ::icu_provider::DataPayload::from_owned(
                                    ::icu_provider::zerofrom::ZeroFrom::zero_from(
                                        litemap_slice_get(#data::DATA, key, req)?
                        )))),
                }
            } else {
                quote!{
                    #feature
                    <#marker as ::icu_provider::ResourceMarker>::KEY =>
                        ::icu_provider::AnyPayload::from_static_ref::<<#marker as ::icu_provider::DataMarker>::Yokeable>(litemap_slice_get(#data::DATA, key, req)?),
                }
            }
        });

        let resource_impls = marker_data_feature.iter().map(|(marker, data, feature)| {
            quote! {
                #feature
                impl_resource_provider!(#marker, #data::DATA);
            }
        });

        self.write_to_file(
            PathBuf::from("mod"),
            quote! {
                #(
                    mod #mods;
                )*

                macro_rules! declare_baked_provider {
                    ($name:ident, impl AnyProvider) => {
                        declare_baked_provider!($name);

                        impl ::icu_provider::AnyProvider for $name {
                            fn load_any(&self, key: ::icu_provider::ResourceKey, req: &::icu_provider::DataRequest) -> Result<::icu_provider::AnyResponse, ::icu_provider::DataError> {
                                Ok(::icu_provider::AnyResponse {
                                    payload: Some(match key {
                                        #(#any_cases)*
                                        _ => return Err(::icu_provider::DataErrorKind::MissingResourceKey.with_req(key, req)),
                                    }),
                                    metadata: Default::default(),
                                })
                            }
                        }
                    };
                    ($name:ident) => {
                        /// This data provider was programmatically generated by [`icu_datagen`](
                        /// https://unicode-org.github.io/icu4x-docs/doc/icu_datagen/enum.Out.html#variant.Module).
                        #[non_exhaustive]
                        pub struct $name;

                        fn litemap_slice_get<T: ?Sized>(
                            values: &'static [(&'static str, &'static T)],
                            key: ::icu_provider::ResourceKey,
                            req: &::icu_provider::DataRequest,
                        ) -> Result<&'static T, ::icu_provider::DataError> {
                            #[allow(clippy::unwrap_used)]
                            values
                                .binary_search_by(|(k, _)| req.options.strict_cmp(k.as_bytes()).reverse())
                                .map(|i| values.get(i).unwrap().1)
                                .map_err(|_| ::icu_provider::DataErrorKind::MissingResourceOptions.with_req(key, req))
                        }

                        macro_rules! impl_resource_provider {
                            ($marker:ty, $data:expr) => {
                                impl ::icu_provider::ResourceProvider<$marker> for $name {
                                    fn load_resource(
                                        &self,
                                        req: &::icu_provider::DataRequest,
                                    ) -> Result<::icu_provider::DataResponse<$marker>, ::icu_provider::DataError> {
                                        Ok(::icu_provider::DataResponse {
                                            metadata: ::icu_provider::DataResponseMetadata::default(),
                                            payload: Some(::icu_provider::DataPayload::from_owned(::icu_provider::zerofrom::ZeroFrom::zero_from(
                                                litemap_slice_get($data, <$marker as ::icu_provider::ResourceMarker>::KEY, req)?,
                                            ))),
                                        })
                                    }
                                }
                            };
                        }

                        #(#resource_impls)*
                    }
                }
            }
        )
        .map_err(|e| e.with_path_context(&PathBuf::from("mod.rs")))?;

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

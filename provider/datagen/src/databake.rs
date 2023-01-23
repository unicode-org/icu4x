// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use databake::{quote, CrateEnv, TokenStream};
use icu_provider::datagen::*;
use icu_provider::prelude::*;
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
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
    // All mod.rs files in the module tree. These can only be written after the last flush.
    mod_files: Mutex<HashMap<PathBuf, BTreeSet<String>>>,
    /// Identifiers of the lookup functions for each key. This is populated by `flush` and consumed by `close`.
    lookup_idents: Mutex<HashMap<SyncTokenStream, SyncTokenStream>>,
    // List of dependencies used by baking.
    dependencies: CrateEnv,
}

impl BakedDataExporter {
    pub fn new(
        mod_directory: PathBuf,
        pretty: bool,
        insert_feature_gates: bool,
        use_separate_crates: bool,
    ) -> Result<Self, DataError> {
        if mod_directory.exists() {
            std::fs::remove_dir(&mod_directory)
                .map_err(|e| DataError::from(e).with_path_context(&mod_directory))?;
        }

        Ok(Self {
            mod_directory,
            pretty,
            insert_feature_gates: insert_feature_gates && use_separate_crates,
            use_separate_crates,
            data: Default::default(),
            mod_files: Default::default(),
            lookup_idents: Default::default(),
            dependencies: Default::default(),
        })
    }

    fn write_to_file<P: AsRef<std::path::Path>>(
        &self,
        relative_path: P,
        data: TokenStream,
        is_expr: bool,
    ) -> Result<(), DataError> {
        let path = self
            .mod_directory
            .join(&relative_path)
            .with_extension(if is_expr { "rs.data" } else { "rs" });

        let mut formatted = if self.pretty {
            use rust_format::*;
            RustFmt::from_config(
                Config::new_str()
                    // We deal with line encoding later
                    .option("newline_style", "unix")
                    .option("normalize_doc_attributes", "true")
                    // Rustfmt silently gives up if it cannot achieve the max width, which happens for the root mod.rs
                    .option(
                        "max_width",
                        if relative_path.as_ref().as_os_str().to_str() == Some("mod") {
                            "150"
                        } else {
                            "100"
                        },
                    ),
            )
            .format_tokens(if is_expr {
                // Rustfmt cannot format Rust expressions, only full files. We need to wrap expressions in a main function
                quote!(fn main() { #data })
            } else {
                data
            })
            .map_err(|e| {
                DataError::custom("Formatting error")
                    .with_display_context(&e)
                    .with_path_context(&path)
            })?
        } else {
            data.to_string()
        };

        if !self.use_separate_crates {
            formatted = formatted
                .replace("icu_", "icu::")
                .replace("icu::provider", "icu_provider");
        }

        let formatted = if self.pretty && is_expr {
            formatted = formatted.replace("\n    ", "\n");
            formatted
                .strip_prefix("fn main() {\n")
                .unwrap()
                .strip_suffix("}\n")
                .unwrap()
        } else {
            &formatted
        };
        std::fs::create_dir_all(&path.parent().unwrap())?;
        let mut file = crlify::BufWriterWithLineEndingFix::new(
            File::create(&path).map_err(|e| DataError::from(e).with_path_context(&path))?,
        );
        if !is_expr {
            writeln!(file, "// @generated")
                .map_err(|e| DataError::from(e).with_path_context(&path))?;
        }
        write!(file, "{formatted}").map_err(|e| DataError::from(e).with_path_context(&path))
    }

    fn print_deps(&mut self) {
        let mut deps = move_out!(self.dependencies)
            .into_iter()
            .collect::<BTreeSet<_>>();
        if !self.use_separate_crates {
            deps.retain(|&krate| krate.starts_with("icu_provider") || !krate.starts_with("icu_"));
            deps.insert("icu");
        }
        deps.insert("icu_provider");
        // TODO: make locale fallback cfg'ed
        deps.insert("icu_provider_adapters");

        log::info!("The generated module requires the following crates:");
        for crate_name in deps {
            log::info!("{}", crate_name);
        }
    }

    fn write_intermediate_mod_files(&mut self) -> Result<(), DataError> {
        move_out!(self.mod_files)
            .into_inner()
            .expect("poison")
            .into_par_iter()
            .try_for_each(|(path, mods)| {
                let mods = mods.into_iter().map(|p| p.parse::<TokenStream>().unwrap());
                self.write_to_file(
                    &path.join("mod"),
                    quote! {
                        #(
                            pub mod #mods;
                        )*
                    },
                    false,
                )
            })?;
        Ok(())
    }

    fn feature_gate(&self, marker: &SyncTokenStream, internal: bool) -> TokenStream {
        let feature = if !self.insert_feature_gates {
            return quote!();
        } else if marker.contains("DateSkeletonPatternsV1Marker") {
            "icu_datetime_experimental"
        } else {
            let feature = marker
                .split(" :: ")
                .next()
                .unwrap()
                .strip_prefix(":: ")
                .unwrap();
            if feature.starts_with("icu_provider") {
                return quote!();
            }
            feature
        };
        if internal {
            quote!(#![cfg(feature = #feature)])
        } else {
            quote!(#[cfg(feature = #feature)])
        }
    }
}

impl DataExporter for BakedDataExporter {
    fn put_payload(
        &self,
        key: DataKey,
        locale: &DataLocale,
        payload: &DataPayload<ExportMarker>,
    ) -> Result<(), DataError> {
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
        let marker = crate::registry::key_to_marker_bake(key, &self.dependencies);
        let feature = self.feature_gate(&marker.to_string(), true);

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

        let mut path = PathBuf::new();
        for level in &module_path.segments {
            self.mod_files
                .lock()
                .expect("poison")
                .entry(path.clone())
                .or_default()
                .insert(level.ident.to_string());
            path = path.join(level.ident.to_string());
        }

        let struct_type = if marker.to_string().contains("DateSkeletonPatternsV1Marker") {
            quote! {
                &'static [(
                    &'static [::icu_datetime::fields::Field],
                    ::icu_datetime::pattern::runtime::PatternPlurals<'static>
                )]
            }
        } else {
            quote! { <#marker as ::icu_provider::DataMarker>::Yokeable }
        };

        let mut map = BTreeMap::new();
        let mut statics = BTreeMap::new();

        let raw = self
            .data
            .lock()
            .expect("poison")
            .remove(&key)
            .unwrap_or_default();

        for (payload_bake_string, locales) in raw {
            let file_name = locales.iter().min().unwrap();
            let ident =
                syn::parse_str::<syn::Ident>(&file_name.to_ascii_uppercase().replace('-', "_"))
                    .unwrap();
            self.write_to_file(
                &path.join(file_name),
                payload_bake_string.parse().unwrap(),
                true,
            )?;
            let file_name = format!("{file_name}.rs.data");
            let statik = quote! { static #ident: DataStruct = include!(#file_name); };
            statics.insert(file_name, statik);
            map.extend(locales.into_iter().map(|l| (l, ident.clone())));
        }

        let (keys, values): (Vec<_>, Vec<_>) = map.into_iter().unzip();

        let lookup = match keys.len() {
            0 => {
                quote! {
                    pub const fn lookup(_: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
                        None
                    }
                }
            }
            1 => {
                let locale = &keys[0];

                let cmp = if locale == "und" {
                    quote! {
                        locale.is_empty()
                    }
                } else if icu_locid::Locale::try_from_bytes_with_single_variant_single_keyword_unicode_extension(locale.as_bytes()).is_ok() {
                    self.dependencies.insert("icu_locid");
                    quote! {
                        icu_provider::DataLocale::from(icu_locid::locale!(#locale)).eq(locale)
                    }
                } else {
                    quote! {
                        locale.strict_cmp(#locale.as_bytes()).is_eq()
                    }
                };
                quote! {
                    pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
                        // This repetition is a singleton
                        #cmp.then(|| #(&#values)*)
                    }
                }
            }
            n => {
                quote! {
                    pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
                        static KEYS: [&str; #n] = [#(#keys),*];
                        static DATA: [&DataStruct; #n] = [#(&#values),*];

                        KEYS
                            .binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse())
                            .ok()
                            .map(|i| unsafe {
                                // Safe because KEYS and DATA have the same length
                                *DATA.get_unchecked(i)
                            })
                    }
                }
            }
        };

        let statics = statics.values();

        self.write_to_file(
            &path.join("mod"),
            quote! {
                #feature

                type DataStruct = #struct_type;

                #lookup

                #(#statics)*
            },
            false,
        )?;

        self.lookup_idents.lock().expect("poison").insert(
            quote!(#marker).to_string(),
            quote!(#module_path :: lookup).to_string(),
        );

        Ok(())
    }

    fn close(&mut self) -> Result<(), DataError> {
        // These are BTreeMaps keyed on the marker to keep the output sorted and stable
        let mut data_impls = BTreeMap::new();
        let mut non_requested_data_impls = BTreeMap::new();
        let mut any_consts = BTreeMap::new();
        let mut any_cases = BTreeMap::new();

        let mut lookup_idents = move_out!(self.lookup_idents).into_inner().expect("poison");

        let ffi_keys = crate::registry::all_keys_for_ffi()
            .into_iter()
            .collect::<HashSet<_>>();

        for key in crate::registry::all_keys_with_experimental()
            .into_iter()
            // HelloWorld is the only key not returned by all_keys
            .chain(std::iter::once(
                icu_provider::hello_world::HelloWorldV1Marker::KEY,
            ))
        {
            let marker = crate::registry::key_to_marker_bake(key, &self.dependencies);
            let marker_str = marker.to_string();
            let feature = self.feature_gate(&marker_str, false);

            if let Some(lookup_ident) = lookup_idents.remove(&marker_str) {
                let lookup_ident = lookup_ident.parse::<TokenStream>().unwrap();

                data_impls.insert(marker_str.clone(),
                    quote! {
                        #feature
                        impl DataProvider<#marker> for $provider {
                            fn load(
                                &self,
                                req: DataRequest,
                            ) -> Result<DataResponse<#marker>, DataError> {
                                #lookup_ident(&req.locale)
                                    .map(zerofrom::ZeroFrom::zero_from)
                                    .map(DataPayload::from_owned)
                                    .map(|payload| {
                                        DataResponse {
                                            metadata: Default::default(),
                                            payload: Some(payload),
                                        }
                                    })
                                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(#marker::KEY, req))
                            }
                        }
                    });

                let hash_ident = marker_str
                    .split(' ')
                    .next_back()
                    .unwrap()
                    .to_ascii_uppercase()
                    .parse::<TokenStream>()
                    .unwrap();

                any_consts.insert(
                    marker_str.clone(),
                    quote! {
                        #feature
                        const #hash_ident: ::icu_provider::DataKeyHash = #marker::KEY.hashed();
                    },
                );
                any_cases.insert(
                    marker_str.clone(),
                    if marker_str
                        == ":: icu_datetime :: provider :: calendar :: DateSkeletonPatternsV1Marker"
                    {
                        quote! {
                            #feature
                            #hash_ident => {
                                #lookup_ident(&req.locale)
                                    .map(zerofrom::ZeroFrom::zero_from)
                                    .map(DataPayload::<#marker>::from_owned)
                                    .map(DataPayload::wrap_into_any_payload)
                            }
                        }
                    } else {
                        quote! {
                            #feature
                            #hash_ident => #lookup_ident(&req.locale).map(AnyPayload::from_static_ref),
                        }
                    },
                );
            } else if ffi_keys.contains(&key) {
                non_requested_data_impls.insert(
                    marker_str.clone(),
                    quote! {
                        #feature
                        impl DataProvider<#marker> for $provider {
                            fn load(
                                &self,
                                req: DataRequest,
                            ) -> Result<DataResponse<#marker>, DataError> {
                                    Err(DataErrorKind::MissingDataKey.with_req(#marker::KEY, req))
                            }
                        }
                    },
                );
            }
        }

        assert!(lookup_idents.is_empty(), "{:?}", lookup_idents);

        let any_code = if any_cases.is_empty() {
            quote! {
                Err(DataErrorKind::MissingDataKey.with_req(key, req))
            }
        } else {
            let any_consts = any_consts.values();
            let any_cases = any_cases.values();
            quote! {
                #(#any_consts)*
                match key.hashed() {
                    #(#any_cases)*
                    _ => return Err(DataErrorKind::MissingDataKey.with_req(key, req)),
                }
                .map(|payload| AnyResponse {
                    payload: Some(payload),
                    metadata: Default::default(),
                })
                .ok_or_else(|| DataErrorKind::MissingLocale.with_req(key, req))
            }
        };

        let mods = self
            .mod_files
            .get_mut()
            .expect("poison")
            .remove(&PathBuf::new())
            .unwrap_or_default()
            .into_iter()
            .map(|p| p.parse::<TokenStream>().unwrap());

        let data_impls = data_impls.values();
        let non_requested_data_impls = non_requested_data_impls.values();

        self.write_to_file(
            PathBuf::from("mod"),
            quote! {
                #(
                    mod #mods;
                )*

                use ::icu_provider::prelude::*;

                /// Implement [`DataProvider<M>`] on the given struct using the data
                /// hardcoded in this module. This allows the struct to be used with
                /// `icu`'s `_unstable` constructors.
                ///
                /// This macro can only be called from its definition-site, i.e. right
                /// after `include!`-ing the generated module.
                ///
                /// ```compile_fail
                /// struct MyDataProvider;
                /// include!("/path/to/generated/mod.rs");
                /// impl_data_provider(MyDataProvider);
                /// ```
                #[allow(unused_macros)]
                macro_rules! impl_data_provider {
                    ($provider:path) => {
                        #(#data_impls)*
                    };
                    ($provider:path, FFI_COMPLETE) => {
                        impl_data_provider!($provider);
                        #(#non_requested_data_impls)*
                    };
                }

                /// Implement [`AnyProvider`] on the given struct using the data
                /// hardcoded in this module. This allows the struct to be used with
                /// `icu`'s `_any` constructors.
                ///
                /// This macro can only be called from its definition-site, i.e. right
                /// after `include!`-ing the generated module.
                /// 
                /// ```compile_fail
                /// struct MyAnyProvider;
                /// include!("/path/to/generated/mod.rs");
                /// impl_any_provider(MyAnyProvider);
                /// ```
                #[allow(unused_macros)]
                macro_rules! impl_any_provider {
                    ($provider:path) => {
                        impl AnyProvider for $provider {
                            fn load_any(&self, key: DataKey, req: DataRequest) -> Result<AnyResponse, DataError> {
                                #any_code
                            }
                        }
                    }
                }

                #[allow(dead_code)]
                struct BakedDataProvider;
                impl_data_provider!(BakedDataProvider);
            },
            false,
        )?;

        self.write_to_file(
            PathBuf::from("any"),
            quote! {
                impl_any_provider!(BakedDataProvider);
            },
            false,
        )?;

        self.write_intermediate_mod_files()?;

        self.print_deps();

        Ok(())
    }
}

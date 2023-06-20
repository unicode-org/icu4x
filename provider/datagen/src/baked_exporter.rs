// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A data exporter that bakes the data into Rust code.
//!
//! This module can be used as a target for the `icu_datagen` crate.
//!
//! # Examples
//!
//! ```
//! use icu_datagen::prelude::*;
//! use icu_datagen::baked_exporter::*;
//!
//! let demo_path = std::env::temp_dir().join("icu4x_baked_demo");
//! # let _ = std::fs::remove_dir_all(&demo_path);
//!
//! // Set up the exporter
//! let mut exporter = BakedExporter::new(demo_path.clone(), Default::default()).unwrap();
//!
//! // Export something
//! DatagenProvider::default()
//!     .export(
//!         [icu_provider::hello_world::HelloWorldV1Marker::KEY].into_iter().collect(),
//!         exporter
//!     ).unwrap();
//! #
//! # let _ = std::fs::remove_dir_all(&demo_path);
//! ```
//!
//! The resulting module structure can now be used like this:
//!
//! ```
//! use icu_locid::langid;
//! use icu_provider::prelude::*;
//! use icu_provider::hello_world::*;
//!
//! pub struct MyDataProvider;
//!
//! mod baked {
//!     # macro_rules! include {
//!     #   ($path:literal) => {}
//!     # }
//!     # macro_rules! impl_data_provider {
//!     #   ($p:path) => {
//!     #     use icu_provider::prelude::*;
//!     #     use icu_provider::hello_world::*;
//!     #     impl DataProvider<HelloWorldV1Marker> for $p {
//!     #       fn load(&self, req: DataRequest) -> Result<DataResponse<HelloWorldV1Marker>, DataError> {
//!     #         HelloWorldProvider.load(req)
//!     #       }
//!     #     }
//!     #   }
//!     # }
//!     include!("/path/to/mod/");
//!     impl_data_provider!(super::MyDataProvider);
//! }
//!
//! # fn main() {
//! let response: DataPayload<HelloWorldV1Marker> = MyDataProvider
//!     .load(DataRequest {
//!         locale: &langid!("en").into(),
//!         metadata: Default::default(),
//!     })
//!     .unwrap()
//!     .take_payload()
//!     .unwrap();
//!
//! assert_eq!(response.get().message, "Hello World");
//! # }
//! ```

use databake::*;
use icu_provider::datagen::*;
use icu_provider::prelude::*;
use std::collections::{BTreeMap, BTreeSet, HashMap};
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

/// Options for configuring the output of [`BakedExporter`].
#[non_exhaustive]
#[derive(Debug)]
pub struct Options {
    /// Whether to run `rustfmt` on the generated files.
    pub pretty: bool,
    /// Whether to gate each key on its crate name. This allows using the module
    /// even if some keys are not required and their dependencies are not included.
    /// Requires use_separate_crates.
    pub insert_feature_gates: bool,
    /// Whether to use separate crates to name types instead of the `icu` metacrate
    pub use_separate_crates: bool,
    /// Whether to overwrite existing data. By default, errors if it is present.
    pub overwrite: bool,
}

#[allow(clippy::derivable_impls)] // want to be explicit about bool defaults
impl Default for Options {
    fn default() -> Self {
        Self {
            pretty: false,
            insert_feature_gates: false,
            use_separate_crates: false,
            overwrite: false,
        }
    }
}

#[allow(clippy::type_complexity)]
/// See the module-level documentation for details.
pub struct BakedExporter {
    // Input arguments
    mod_directory: PathBuf,
    pretty: bool,
    insert_feature_gates: bool,
    use_separate_crates: bool,
    // Temporary storage for put_payload: key -> (bake -> {locale})
    data: Mutex<HashMap<DataKey, BTreeMap<SyncTokenStream, BTreeSet<String>>>>,
    /// Information to generate implementations. This is populated by `flush` and consumed by `close`.
    impl_data: Mutex<BTreeMap<&'static str, ImplData>>,
    // List of dependencies used by baking.
    dependencies: CrateEnv,
}

/// Data required to write the implementations
struct ImplData {
    marker: SyncTokenStream,
    feature: SyncTokenStream,
    macro_ident: SyncTokenStream,
    prefixed_macro_ident: SyncTokenStream,
    hash_ident: SyncTokenStream,
    mod_ident: SyncTokenStream,
}

impl std::fmt::Debug for BakedExporter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BakedExporter")
            .field("mod_directory", &self.mod_directory)
            .field("pretty", &self.pretty)
            .field("insert_feature_gates", &self.insert_feature_gates)
            .field("use_separate_crates", &self.use_separate_crates)
            // skip formatting intermediate data
            .finish()
    }
}

impl BakedExporter {
    /// Constructs a new [`BakedExporter`] with the given output directory and options.
    pub fn new(mod_directory: PathBuf, options: Options) -> Result<Self, DataError> {
        let Options {
            pretty,
            insert_feature_gates,
            use_separate_crates,
            overwrite,
        } = options;

        if mod_directory.exists() {
            if overwrite {
                std::fs::remove_dir_all(&mod_directory)
            } else {
                std::fs::remove_dir(&mod_directory)
            }
            .map_err(|e| DataError::from(e).with_path_context(&mod_directory))?;
        }

        Ok(Self {
            mod_directory,
            pretty,
            insert_feature_gates: insert_feature_gates && use_separate_crates,
            use_separate_crates,
            data: Default::default(),
            impl_data: Default::default(),
            dependencies: Default::default(),
        })
    }

    fn write_to_file<P: AsRef<std::path::Path>>(
        &self,
        relative_path: P,
        data: TokenStream,
    ) -> Result<(), DataError> {
        let path = self.mod_directory.join(&relative_path);

        let mut formatted = if self.pretty {
            use std::process::{Command, Stdio};
            let mut rustfmt = Command::new("rustfmt")
                .arg("--config")
                .arg("newline_style=unix")
                .arg("--config")
                .arg("normalize_doc_attributes=true")
                .arg("--config")
                .arg("max_width=5000000") // better to format wide than to not format
                // currently unnecessary, may become necessary for format_macro_bodies
                // in the future
                .arg("--config")
                .arg("unstable_features=true")
                .arg("--config")
                .arg("format_macro_bodies=true")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?;
            let mut rustfmt_stdin = rustfmt.stdin.take().unwrap();
            write!(rustfmt_stdin, "{data}")?;

            drop(rustfmt_stdin); // EOF

            let output = rustfmt.wait_with_output()?;
            if !output.status.success() {
                let stderr = String::from_utf8(output.stderr)
                    .map_err(|_| DataError::custom("rustfmt output not utf-8"))?;
                return Err(DataError::custom("rustfmt failed").with_display_context(&stderr));
            }
            String::from_utf8(output.stdout)
                .map_err(|_| DataError::custom("rustfmt output not utf-8"))?
        } else {
            data.to_string()
        };

        if !self.use_separate_crates {
            formatted = formatted
                .replace("icu_", "icu::")
                .replace("icu::provider", "icu_provider");
        }

        std::fs::create_dir_all(path.parent().unwrap())?;
        let mut file = crlify::BufWriterWithLineEndingFix::new(
            File::create(&path).map_err(|e| DataError::from(e).with_path_context(&path))?,
        );
        write!(file, "// @generated\n{formatted}")
            .map_err(|e| DataError::from(e).with_path_context(&path))
    }

    fn print_deps(&mut self) {
        let mut deps = move_out!(self.dependencies)
            .into_iter()
            .collect::<BTreeSet<_>>();
        if !self.use_separate_crates {
            deps.retain(|&krate| !krate.starts_with("icu_"));
            deps.insert("icu");
        }
        deps.insert("icu_provider");

        log::info!("The generated module requires the following crates:");
        for crate_name in deps {
            log::info!("{}", crate_name);
        }
    }
}

impl DataExporter for BakedExporter {
    fn put_payload(
        &self,
        key: DataKey,
        locale: &DataLocale,
        payload: &DataPayload<ExportMarker>,
    ) -> Result<(), DataError> {
        let payload = payload.tokenize(&self.dependencies);
        let payload = payload.to_string();
        let locale = locale.to_string();
        self.data
            .lock()
            .expect("poison")
            .entry(key)
            .or_default()
            .entry(payload)
            .or_default()
            .insert(locale);
        Ok(())
    }

    fn flush(&self, key: DataKey) -> Result<(), DataError> {
        let marker =
            syn::parse2::<syn::Path>(crate::registry::key_to_marker_bake(key, &self.dependencies))
                .unwrap();

        let is_datetime_skeletons =
            marker.segments.iter().next_back().unwrap().ident == "DateSkeletonPatternsV1Marker";

        let feature = if !self.insert_feature_gates {
            quote!()
        } else if is_datetime_skeletons {
            quote! { #[cfg(feature = "icu_datetime_experimental")] }
        } else {
            let feature = marker.segments.iter().next().unwrap().ident.to_string();
            if !feature.starts_with("icu_provider") {
                quote! { #[cfg(feature = #feature)] }
            } else {
                quote!()
            }
        };

        let values = self.data.lock().expect("poison").remove(&key);

        let values = values
            .unwrap_or_default()
            .into_iter()
            .map(|(payload_bake_string, locales)| {
                (payload_bake_string.parse::<TokenStream>().unwrap(), locales)
            })
            .collect::<Vec<_>>();

        let struct_type = if is_datetime_skeletons {
            quote! {
                &'static [(
                    &'static [icu_datetime::fields::Field],
                    icu_datetime::pattern::runtime::PatternPlurals<'static>
                )]
            }
        } else {
            quote! { <#marker as icu_provider::DataMarker>::Yokeable }
        };

        let ident = key
            .path()
            .to_ascii_lowercase()
            .replace('@', "_v")
            .replace('/', "_");

        let mut singleton = None.into_iter();

        let lookup = match values.iter().map(|(_, l)| l.len()).sum() {
            0 => quote!(Err(icu_provider::DataErrorKind::MissingLocale)),
            1 => {
                let (bake, locale) = values.into_iter().next().unwrap();
                let locale = locale.into_iter().next().unwrap();

                let singleton_ident = format!("SINGLETON_{}", ident.to_ascii_uppercase())
                    .parse::<TokenStream>()
                    .unwrap();

                // Exposing singleton structs separately allows us to get rid of fallibility by using
                // the struct directly.
                singleton = Some(quote! {
                    #[clippy::msrv = "1.61"]
                    impl $provider {
                        #[doc(hidden)]
                        pub const #singleton_ident: &'static #struct_type = &#bake;
                    }
                })
                .into_iter();

                if locale == "und" {
                    quote! {
                        if locale.is_empty() {
                            Ok(Self::#singleton_ident)
                        } else {
                            Err(icu_provider::DataErrorKind::ExtraneousLocale)
                        }
                    }
                } else if icu_locid::Locale::try_from_bytes_with_single_variant_single_keyword_unicode_extension(locale.as_bytes()).is_ok() {
                    self.dependencies.insert("icu_locid");
                    quote! {
                        if icu_provider::DataLocale::from(icu_locid::locale!(#locale)).eq(&locale) {
                            Ok(Self::#singleton_ident)
                        } else {
                            Err(icu_provider::DataErrorKind::MissingLocale)
                        }
                    }
                } else {
                    quote! {
                        if locale.strict_cmp(#locale.as_bytes()).is_eq() {
                            Ok(Self::#singleton_ident)
                        } else {
                            Err(icu_provider::DataErrorKind::MissingLocale)
                        }
                    }
                }
            }
            _ => {
                let mut map = BTreeMap::new();
                let mut statics = Vec::new();

                for (bake, locales) in values {
                    let first_locale = locales.iter().next().unwrap();
                    let anchor = syn::parse_str::<syn::Ident>(
                        &first_locale.to_ascii_uppercase().replace('-', "_"),
                    )
                    .unwrap();
                    statics.push(quote! { static #anchor: #struct_type = #bake; });
                    map.extend(locales.into_iter().map(|l| (l, anchor.clone())));
                }

                let (keys, values): (Vec<_>, Vec<_>) = map.into_iter().unzip();

                quote! {
                    #(#statics)*
                    match [#(#keys),*].binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse()) {
                        Ok(i) => Ok(*unsafe {
                            // Safe because keys and values have the same length
                            [#(&#values),*].get_unchecked(i)
                        }),
                        Err(_) => Err(icu_provider::DataErrorKind::MissingLocale)
                    }
                }
            }
        };

        let into_data_payload = if is_datetime_skeletons {
            quote! {
                icu_provider::DataPayload::from_owned(icu_datetime::provider::calendar::DateSkeletonPatternsV1(
                    payload
                        .iter()
                        .map(|(fields, pattern)| (
                            icu_datetime::provider::calendar::SkeletonV1((*fields).into()),
                            icu_provider::prelude::zerofrom::ZeroFrom::zero_from(pattern)
                        ))
                        .collect(),
                ))

            }
        } else {
            quote! {
                icu_provider::DataPayload::from_static_ref(payload)
            }
        };

        let doc = format!(
            " Implement [`DataProvider<{}>`](icu_provider::DataProvider) on the given struct using the data",
            marker.segments.iter().next_back().unwrap().ident
        );
        let prefixed_macro_ident = format!("__impl_{ident}").parse::<TokenStream>().unwrap();
        self.write_to_file(
            PathBuf::from(format!("macros/{}.data.rs", ident)),
            quote!{
                #[doc = #doc]
                /// hardcoded in this file. This allows the struct to be used with
                /// `icu`'s `_unstable` constructors.
                #[doc(hidden)]
                #[macro_export]
                macro_rules! #prefixed_macro_ident {
                    ($provider:path) => {
                        #(#singleton)*

                        #[clippy::msrv = "1.61"]
                        impl icu_provider::DataProvider<#marker> for $provider {
                            fn load(
                                &self,
                                req: icu_provider::DataRequest,
                            ) -> Result<icu_provider::DataResponse<#marker>, icu_provider::DataError> {
                                let locale = req.locale;
                                match {#lookup} {
                                    Ok(payload) => Ok(icu_provider::DataResponse {
                                        metadata: Default::default(),
                                        payload: Some(#into_data_payload),
                                    }),
                                    Err(e) => Err(e.with_req(<#marker as icu_provider::KeyedDataMarker>::KEY, req))
                                }
                            }
                        }
                    }
                }
            }
        )?;

        let data = ImplData {
            feature: feature.to_string(),
            marker: quote!(#marker).to_string(),
            macro_ident: format!("impl_{ident}"),
            prefixed_macro_ident: prefixed_macro_ident.to_string(),
            hash_ident: ident.to_ascii_uppercase(),
            mod_ident: ident,
        };

        self.impl_data
            .lock()
            .expect("poison")
            .insert(key.path().get(), data);

        Ok(())
    }

    fn close(&mut self) -> Result<(), DataError> {
        log::info!("Writing macros module...");

        let data = move_out!(self.impl_data).into_inner().expect("poison");

        let features = data
            .values()
            .map(|data| data.feature.parse::<TokenStream>().unwrap())
            .collect::<Vec<_>>();
        let markers = data
            .values()
            .map(|data| data.marker.parse::<TokenStream>().unwrap())
            .collect::<Vec<_>>();

        let macro_idents = data
            .values()
            .map(|data| data.macro_ident.parse::<TokenStream>().unwrap())
            .collect::<Vec<_>>();
        let mod_idents = data
            .values()
            .map(|data| data.mod_ident.parse::<TokenStream>().unwrap())
            .collect::<Vec<_>>();
        let file_paths = data
            .values()
            .map(|data| format!("macros/{}.data.rs", data.mod_ident))
            .collect::<Vec<_>>();

        // We prefix all macros with `__`, as these will be automatically exported at the crate root, which is annoying
        // for crates that include the data but don't want it to be public. We then reexport them as items that use
        // normal scoping that clients can control.
        let prefixed_macro_idents = data
            .values()
            .map(|data| data.prefixed_macro_ident.parse::<TokenStream>().unwrap())
            .collect::<Vec<_>>();

        let hash_idents = data
            .values()
            .map(|data| data.hash_ident.parse::<TokenStream>().unwrap())
            .collect::<Vec<_>>();

        // macros.rs is the interface for built-in data. It exposes one macro per data key.
        self.write_to_file(
            PathBuf::from("macros.rs"),
            quote! {
                #(
                    #[macro_use]
                    #[path = #file_paths]
                    mod #mod_idents;
                    #[doc(inline)]
                    pub use #prefixed_macro_idents as #macro_idents;
                )*
            },
        )?;

        // mod.rs is the interface for using databake directly. It exposes the macros from macros.rs,
        // as well as `impl_data_provider` and `impl_any_provider` which include all keys.
        self.write_to_file(
            PathBuf::from("mod.rs"),
            quote! {
                include!("macros.rs");

                /// Implement [`DataProvider<M>`](icu_provider::DataProvider) on the given struct using the data
                /// hardcoded in this module. This allows the struct to be used with
                /// `icu`'s `_unstable` constructors.
                ///
                /// ```compile_fail
                /// struct MyDataProvider;
                /// include!("/path/to/generated/mod.rs");
                /// impl_data_provider(MyDataProvider);
                /// ```
                #[doc(hidden)]
                #[macro_export]
                macro_rules! __impl_data_provider {
                    ($provider:path) => {
                        #(
                            #features
                            #macro_idents ! ($provider);
                        )*
                    }
                }
                #[doc(inline)]
                pub use __impl_data_provider as impl_data_provider;

                /// Implement [`AnyProvider`](icu_provider::AnyProvider) on the given struct using the data
                /// hardcoded in this module. This allows the struct to be used with
                /// `icu`'s `_any` constructors.
                ///
                /// ```compile_fail
                /// struct MyAnyProvider;
                /// include!("/path/to/generated/mod.rs");
                /// impl_any_provider(MyAnyProvider);
                /// ```
                #[doc(hidden)]
                #[macro_export]
                macro_rules! __impl_any_provider {
                    ($provider:path) => {
                        #[clippy::msrv = "1.61"]
                        impl icu_provider::AnyProvider for $provider {
                            fn load_any(&self, key: icu_provider::DataKey, req: icu_provider::DataRequest) -> Result<icu_provider::AnyResponse, icu_provider::DataError> {
                                #(
                                    #features
                                    const #hash_idents: icu_provider::DataKeyHash = <#markers as icu_provider::KeyedDataMarker>::KEY.hashed();
                                )*
                                match key.hashed() {
                                    #(
                                        #features
                                        #hash_idents => icu_provider::DataProvider::<#markers>::load(self, req)
                                            .and_then(|r| r.take_metadata_and_payload())
                                            .map(|(metadata, payload)| icu_provider::AnyResponse {
                                                payload: Some(payload.wrap_into_any_payload()),
                                                metadata,
                                            }),
                                    )*
                                    _ => Err(icu_provider::DataErrorKind::MissingDataKey.with_req(key, req)),
                                }
                            }
                        }
                    }
                }
                #[doc(inline)]
                pub use __impl_any_provider as impl_any_provider;

                #[clippy::msrv = "1.61"]
                pub struct BakedDataProvider;
                impl_data_provider!(BakedDataProvider);
            },
        )?;

        // For backwards compatibility
        self.write_to_file(
            PathBuf::from("any.rs"),
            quote! {
                // This assumes that `mod.rs` is already included.
                impl_any_provider!(BakedDataProvider);
            },
        )?;

        self.print_deps();

        Ok(())
    }
}

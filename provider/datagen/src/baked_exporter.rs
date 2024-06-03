// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A data exporter that bakes the data into Rust code.
//!
//! This module can be used as a target for the `icu_datagen` crate.
//!
//! See our [datagen tutorial](https://github.com/unicode-org/icu4x/blob/main/tutorials/data_management.md) for more information about different data providers.
//!
//! # Examples
//!
//! ```
//! use icu_datagen::baked_exporter::*;
//! use icu_datagen::prelude::*;
//!
//! let demo_path = std::env::temp_dir().join("icu4x_baked_demo");
//! # let _ = std::fs::remove_dir_all(&demo_path);
//!
//! // Set up the exporter
//! let mut exporter =
//!     BakedExporter::new(demo_path.clone(), Default::default()).unwrap();
//!
//! // Export something
//! DatagenDriver::new()
//!     .with_markers([icu_provider::hello_world::HelloWorldV1Marker::INFO])
//!     .with_locales_and_fallback([LocaleFamily::FULL], {
//!         let mut options = FallbackOptions::default();
//!         // HelloWorldProvider cannot provide fallback data, so we cannot deduplicate
//!         options.deduplication_strategy = Some(DeduplicationStrategy::None);
//!         options
//!     })
//!     .export(&icu_provider::hello_world::HelloWorldProvider, exporter)
//!     .unwrap();
//! #
//! # let _ = std::fs::remove_dir_all(&demo_path);
//! ```
//!
//! There are two ways to use baked data: you can build custom data providers for use with
//! [`_unstable` constructors](icu_provider::constructors), or you can use it with the
//! `compiled_data` Cargo feature and constructors.
//!
//! ## Custom `DataProvider`
//!
//! This allows you to use baked data in custom data pipelines, such as including some baked
//! data and lazily loading more data from the network.
//!
//! ```
//! use icu_locale_core::locale;
//! use icu_provider::hello_world::*;
//!
//! # macro_rules! include {
//! #   ($path:literal) => {}
//! # }
//! # macro_rules! impl_data_provider {
//! #   ($p:ty) => {
//! #     use icu_provider::prelude::*;
//! #     use icu_provider::hello_world::*;
//! #     impl DataProvider<HelloWorldV1Marker> for $p {
//! #       fn load(&self, req: DataRequest) -> Result<DataResponse<HelloWorldV1Marker>, DataError> {
//! #         HelloWorldProvider.load(req)
//! #       }
//! #     }
//! #   }
//! # }
//! include!("/tmp/icu4x_baked_demo/mod.rs");
//!
//! pub struct MyDataProvider;
//! impl_data_provider!(MyDataProvider);
//!
//! # fn main() {
//! let formatter = HelloWorldFormatter::try_new_unstable(&MyDataProvider, &locale!("en").into()).unwrap();
//!
//! assert_eq!(formatter.format_to_string(), "Hello World");
//! # }
//! ```
//!
//! ## `compiled_data`
//!
//! You can use baked data to overwrite the compiled data that's included in ICU4X.
//! To do this, build your binary with the `ICU4X_DATA_DIR` environment variable:
//!
//! ```console
//! ICU4X_DATA_DIR=/tmp/icu4x_baked_demo cargo build <...>
//! ```
//!
//! ```
//! use icu_locale_core::locale;
//! use icu_provider::hello_world::*;
//!
//! let formatter =
//!     HelloWorldFormatter::try_new(&locale!("en").into()).unwrap();
//!
//! assert_eq!(formatter.format_to_string(), "Hello World");
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

// Produces an MSRV clippy annotation if the `CARGO_PKG_RUST_VERSION` is set.
fn maybe_msrv() -> TokenStream {
    std::option_env!("CARGO_PKG_RUST_VERSION")
        .map(|msrv| {
            quote! {
                #[clippy::msrv = #msrv]
            }
        })
        .unwrap_or_default()
}

/// Options for configuring the output of [`BakedExporter`].
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub struct Options {
    /// Whether to run `rustfmt` on the generated files.
    pub pretty: bool,
    /// Whether to use separate crates to name types instead of the `icu` metacrate.
    ///
    /// By default, types will be named through the `icu` crate, like `icu::list::provider::ListJoinerPattern`.
    /// With this enabled, the alternative name from the component crates will be used: `icu_list::provider::ListJoinerPattern`.
    /// This is required when you are not using the `icu` crate, *and* you're building custom data providers;
    /// data for `compiled_data` constructors uses `icu` names.
    pub use_separate_crates: bool,
    /// Whether to overwrite existing data. By default, errors if it is present.
    pub overwrite: bool,
}

#[allow(clippy::derivable_impls)] // want to be explicit about bool defaults
impl Default for Options {
    fn default() -> Self {
        Self {
            pretty: false,
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
    use_separate_crates: bool,
    // Temporary storage for put_payload: marker -> (bake -> {(locale, marker_attributes)})
    data: Mutex<HashMap<DataMarkerInfo, BTreeMap<SyncTokenStream, BTreeSet<(String, String)>>>>,
    /// (Key, Marker) pairs to wire up in mod.rs. This is populated by `flush` and consumed by `close`.
    impl_data: Mutex<BTreeMap<DataMarkerInfo, SyncTokenStream>>,
    // List of dependencies used by baking.
    dependencies: CrateEnv,
}

impl std::fmt::Debug for BakedExporter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BakedExporter")
            .field("mod_directory", &self.mod_directory)
            .field("pretty", &self.pretty)
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
                .replace("icu::provider", "icu_provider")
                .replace("icu::locale_core", "icu_locale_core")
                .replace("icu::pattern", "icu_pattern");
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

    fn write_impl_macros(
        &self,
        body: TokenStream,
        iterable_body: TokenStream,
        marker: DataMarkerInfo,
        marker_bake: TokenStream,
    ) -> Result<(), DataError> {
        let marker_string = marker_bake.to_string();
        let marker_last = marker_bake.into_iter().last().unwrap();
        let doc = format!(
            " Implement `DataProvider<{}>` on the given struct using the data",
            marker_last
        );
        let doc_iterable = format!(
            " Implement `IterableDataProvider<{}>` on the given struct using the data",
            marker_last
        );

        let ident = Self::ident(marker);

        let prefixed_macro_ident = format!("__impl_{ident}").parse::<TokenStream>().unwrap();
        let prefixed_macro_ident_iterable = format!("__impliterable_{ident}")
            .parse::<TokenStream>()
            .unwrap();

        let maybe_msrv = maybe_msrv();

        self.write_to_file(
            PathBuf::from(format!("macros/{}.rs.data", ident)),
            quote! {
                #[doc = #doc]
                /// hardcoded in this file. This allows the struct to be used with
                /// `icu`'s `_unstable` constructors.
                #[doc(hidden)] // macro
                #[macro_export]
                macro_rules! #prefixed_macro_ident {
                    ($provider:ty) => {
                        #maybe_msrv
                        const _: () = <$provider>::MUST_USE_MAKE_PROVIDER_MACRO;
                        #body
                    }
                }
                #[doc = #doc_iterable]
                /// hardcoded in this file. This allows the struct to be used with
                /// `DatagenDriver` for this marker.
                #[doc(hidden)] // macro
                #[macro_export]
                macro_rules! #prefixed_macro_ident_iterable {
                    ($provider:ty) => {
                        #iterable_body
                    }
                }
            },
        )?;

        self.impl_data
            .lock()
            .expect("poison")
            .insert(marker, marker_string);
        Ok(())
    }

    fn ident(marker: DataMarkerInfo) -> String {
        marker
            .path
            .to_ascii_lowercase()
            .replace('@', "_v")
            .replace('/', "_")
    }
}

impl DataExporter for BakedExporter {
    fn put_payload(
        &self,
        marker: DataMarkerInfo,
        locale: &DataLocale,
        marker_attributes: &DataMarkerAttributes,
        payload: &DataPayload<ExportMarker>,
    ) -> Result<(), DataError> {
        let payload = payload.tokenize(&self.dependencies);
        let payload = payload.to_string();
        let locale = locale.to_string();
        let marker_attributes = marker_attributes.to_string();
        self.data
            .lock()
            .expect("poison")
            .entry(marker)
            .or_default()
            .entry(payload)
            .or_default()
            .insert((locale, marker_attributes));
        Ok(())
    }

    fn flush_singleton(
        &self,
        marker: DataMarkerInfo,
        payload: &DataPayload<ExportMarker>,
    ) -> Result<(), DataError> {
        let marker_bake = bake_marker(marker, &self.dependencies);

        let singleton_ident = format!("SINGLETON_{}", Self::ident(marker).to_ascii_uppercase())
            .parse::<TokenStream>()
            .unwrap();

        let bake = payload.tokenize(&self.dependencies);

        let maybe_msrv = maybe_msrv();

        self.write_impl_macros(quote! {
            #maybe_msrv
            impl $provider {
                // Exposing singleton structs as consts allows us to get rid of fallibility
                #[doc(hidden)] // singletons might be used cross-crate
                pub const #singleton_ident: &'static <#marker_bake as icu_provider::DynDataMarker>::Yokeable = &#bake;
            }

            #maybe_msrv
            impl icu_provider::DataProvider<#marker_bake> for $provider {
                fn load(
                    &self,
                    req: icu_provider::DataRequest,
                ) -> Result<icu_provider::DataResponse<#marker_bake>, icu_provider::DataError> {
                    if req.locale.is_empty() {
                        Ok(icu_provider::DataResponse {
                            payload: Some(icu_provider::DataPayload::from_static_ref(Self::#singleton_ident)),
                            metadata: Default::default(),
                        })
                    } else {
                        Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<#marker_bake as icu_provider::DataMarker>::INFO, req))
                    }
                }
            }
        }, quote! {
            #maybe_msrv
            impl icu_provider::datagen::IterableDataProvider<#marker_bake> for $provider {
                fn supported_requests(&self) -> Result<std::collections::HashSet<(icu_provider::DataLocale, icu_provider::DataMarkerAttributes)>, icu_provider::DataError> {
                    Ok(HashSet::from_iter([Default::default()]))
                }
            }
        }, marker, marker_bake)
    }

    fn flush(&self, marker: DataMarkerInfo) -> Result<(), DataError> {
        self.flush_internal(marker, None)
    }

    fn flush_with_built_in_fallback(
        &self,
        marker: DataMarkerInfo,
        fallback_mode: BuiltInFallbackMode,
    ) -> Result<(), DataError> {
        self.flush_internal(marker, Some(fallback_mode))
    }

    fn close(&mut self) -> Result<(), DataError> {
        self.close_internal()
    }

    fn supports_built_in_fallback(&self) -> bool {
        true
    }
}

impl BakedExporter {
    fn flush_internal(
        &self,
        marker: DataMarkerInfo,
        fallback_mode: Option<BuiltInFallbackMode>,
    ) -> Result<(), DataError> {
        let marker_bake = bake_marker(marker, &self.dependencies);

        let (struct_type, into_data_payload) = if marker_bake
            .to_string()
            .trim()
            .ends_with("DateSkeletonPatternsV1Marker")
        {
            (
                quote! {
                    &'static [(
                        &'static [icu_datetime::fields::Field],
                        icu_datetime::pattern::runtime::PatternPlurals<'static>
                    )]
                },
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

                },
            )
        } else {
            (
                quote!(<#marker_bake as icu_provider::DynDataMarker>::Yokeable),
                quote!(icu_provider::DataPayload::from_static_ref(payload)),
            )
        };

        let values = self
            .data
            .lock()
            .expect("poison")
            .remove(&marker)
            .unwrap_or_default();

        let maybe_msrv = maybe_msrv();

        let (load_body, iterable_body) = if values.is_empty() {
            (
                quote!(Err(icu_provider::DataErrorKind::MissingLocale.with_req(<#marker_bake as icu_provider::DataMarker>::INFO, req))),
                quote!(Ok(Default::default())),
            )
        } else {
            let mut map = BTreeMap::new();
            let mut statics = Vec::new();

            for (bake, reqs) in values {
                let (first_locale, first_marker_attributes) = reqs.iter().next().unwrap();
                let anchor = proc_macro2::Ident::new(
                    &DataRequest {
                        locale: &first_locale.parse().unwrap(),
                        marker_attributes: &first_marker_attributes.parse().unwrap(),
                        ..Default::default()
                    }
                    .legacy_encode()
                    .chars()
                    .map(|ch| {
                        if ch == '-' {
                            '_'
                        } else {
                            ch.to_ascii_uppercase()
                        }
                    })
                    .collect::<String>(),
                    proc_macro2::Span::call_site(),
                );
                let bake = bake.parse::<TokenStream>().unwrap();
                statics.push(quote! { static #anchor: #struct_type = #bake; });
                map.extend(reqs.into_iter().map(|(l, a)| {
                    (
                        DataRequest {
                            locale: &l.parse().unwrap(),
                            marker_attributes: &a.parse().unwrap(),
                            ..Default::default()
                        }
                        .legacy_encode(),
                        anchor.clone(),
                    )
                }));
            }

            let (markers, values): (Vec<_>, Vec<_>) = map.into_iter().unzip();

            let n = markers.len();

            statics.push(quote!(static VALUES: [& #struct_type; #n] = [#(&#values),*];));

            statics.push(quote!(static KEYS: [&str; #n] = [#(#markers),*];));

            let load_body = match fallback_mode {
                None => {
                    quote! {
                        #(#statics)*
                        if let Ok(payload) = KEYS
                                .binary_search_by(|k| req.legacy_cmp(k.as_bytes()).reverse())
                                .map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                            Ok(icu_provider::DataResponse {
                                payload: Some(#into_data_payload),
                                metadata: Default::default(),
                            })
                        } else {
                            Err(icu_provider::DataErrorKind::MissingLocale.with_req(<#marker_bake as icu_provider::DataMarker>::INFO, req))
                        }
                    }
                }
                Some(BuiltInFallbackMode::Standard) => {
                    self.dependencies.insert("icu_locale/compiled_data");
                    quote! {
                        #(#statics)*

                        let mut metadata = icu_provider::DataResponseMetadata::default();

                        let payload =  if let Ok(payload) = KEYS
                                .binary_search_by(|k| req.legacy_cmp(k.as_bytes()).reverse())
                                .map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                            payload
                        } else {
                            const FALLBACKER: icu_locale::fallback::LocaleFallbackerWithConfig<'static> =
                                icu_locale::fallback::LocaleFallbacker::new()
                                    .for_config(<#marker_bake as icu_provider::DataMarker>::INFO.fallback_config);
                            let mut fallback_iterator = FALLBACKER.fallback_for(req.locale.clone());
                            loop {
                                if let Ok(payload) = KEYS
                                        .binary_search_by(|k| icu_provider::DataRequest { locale: fallback_iterator.get(), ..req }.legacy_cmp(k.as_bytes()).reverse())
                                        .map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                                    metadata.locale = Some(fallback_iterator.take());
                                    break payload;
                                }
                                if fallback_iterator.get().is_und() {
                                    return Err(icu_provider::DataErrorKind::MissingLocale.with_req(<#marker_bake as icu_provider::DataMarker>::INFO, req));
                                }
                                fallback_iterator.step();
                            }
                        };

                        Ok(icu_provider::DataResponse {
                            payload: Some(#into_data_payload),
                            metadata
                        })
                    }
                }
                f => {
                    return Err(DataError::custom("Unknown fallback mode")
                        .with_display_context(&format!("{f:?}")))
                }
            };

            let iterable_body = quote!(Ok(
                [#(#markers),*]
                .into_iter()
                .filter_map(icu_provider::DataRequest::legacy_decode)
                .collect()
            ));

            (load_body, iterable_body)
        };

        self.write_impl_macros(
            quote! {
                #maybe_msrv
                impl icu_provider::DataProvider<#marker_bake> for $provider {
                    fn load(
                        &self,
                        req: icu_provider::DataRequest,
                    ) -> Result<icu_provider::DataResponse<#marker_bake>, icu_provider::DataError> {
                        #load_body
                    }
                }
            },
            quote! {
                #maybe_msrv
                impl icu_provider::datagen::IterableDataProvider<#marker_bake> for $provider {
                    fn supported_requests(&self) -> Result<std::collections::HashSet<(icu_provider::DataLocale, icu_provider::DataMarkerAttributes)>, icu_provider::DataError> {
                        #iterable_body
                    }
                }
            },
            marker,
            marker_bake,
        )
    }

    fn close_internal(&mut self) -> Result<(), DataError> {
        log::info!("Writing macros module...");

        let data = move_out!(self.impl_data).into_inner().expect("poison");

        let marker_bakes = data
            .values()
            .map(|marker| marker.parse::<TokenStream>().unwrap())
            .collect::<Vec<_>>();

        let (
            macro_idents,
            prefixed_macro_idents,
            macro_idents_iterable,
            prefixed_macro_idents_iterable,
            mod_idents,
            file_paths,
        ): (Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>) =
            itertools::multiunzip(data.keys().map(|&marker| {
                let ident = Self::ident(marker);
                (
                    format!("impl_{}", ident).parse::<TokenStream>().unwrap(),
                    // We prefix all macros with `__`, as these will be automatically exported at the crate root, which is annoying
                    // for crates that include the data but don't want it to be public. We then reexport them as items that use
                    // normal scoping that clients can control.
                    format!("__impl_{}", ident).parse::<TokenStream>().unwrap(),
                    format!("impliterable_{}", ident)
                        .parse::<TokenStream>()
                        .unwrap(),
                    format!("__impliterable_{}", ident)
                        .parse::<TokenStream>()
                        .unwrap(),
                    ident.parse::<TokenStream>().unwrap(),
                    format!("macros/{}.rs.data", ident),
                )
            }));

        let maybe_msrv = maybe_msrv();

        // macros.rs is the interface for built-in data. It exposes one macro per marker.
        self.write_to_file(
            PathBuf::from("macros.rs"),
            quote! {
                /// Marks a type as a data provider. You can then use macros like
                /// `impl_core_helloworld_v1` to add implementations.
                ///
                /// ```ignore
                /// struct MyProvider;
                /// const _: () = {
                ///     include!("path/to/generated/macros.rs");
                ///     make_provider!(MyProvider);
                ///     impl_core_helloworld_v1!(MyProvider);
                /// }
                /// ```
                #[doc(hidden)] // macro
                #[macro_export]
                macro_rules! __make_provider {
                    ($name:ty) => {
                        #maybe_msrv
                        impl $name {
                            #[allow(dead_code)]
                            pub(crate) const MUST_USE_MAKE_PROVIDER_MACRO: () = ();
                        }
                        icu_provider::impl_data_provider_never_marker!($name);
                    };
                }
                #[doc(inline)]
                pub use __make_provider as make_provider;
                #(
                    #[macro_use]
                    #[path = #file_paths]
                    mod #mod_idents;
                    #[doc(inline)]
                    pub use #prefixed_macro_idents as #macro_idents;
                    #[doc(inline)]
                    pub use #prefixed_macro_idents_iterable as #macro_idents_iterable;
                )*
            },
        )?;

        // mod.rs is the interface for using databake directly. It exposes the macros from macros.rs,
        // as well as `impl_data_provider` and `impl_any_provider` which include all markers.
        self.write_to_file(
            PathBuf::from("mod.rs"),
            quote! {
                include!("macros.rs");

                // Not public as it will only work locally due to needing access to the macros from `macros.rs`.
                macro_rules! impl_data_provider {
                    ($provider:ty) => {
                        make_provider!($provider);
                        #(
                            #macro_idents ! ($provider);
                        )*
                    };
                }

                // Not public because `impl_data_provider` isn't. Users can implement `DynamicDataProvider<AnyMarker>`
                // using `impl_dynamic_data_provider!`.
                #[allow(unused_macros)]
                macro_rules! impl_any_provider {
                    ($provider:ty) => {
                        #maybe_msrv
                        impl icu_provider::AnyProvider for $provider {
                            fn load_any(&self, marker: icu_provider::DataMarkerInfo, req: icu_provider::DataRequest) -> Result<icu_provider::AnyResponse, icu_provider::DataError> {
                                match marker.path.hashed() {
                                    #(
                                        h if h == <#marker_bakes as icu_provider::DataMarker>::INFO.path.hashed() =>
                                            icu_provider::DataProvider::<#marker_bakes>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                                    )*
                                    _ => Err(icu_provider::DataErrorKind::MissingDataMarker.with_req(marker, req)),
                                }
                            }
                        }
                    }
                }

                // For backwards compatibility
                #maybe_msrv
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

macro_rules! cb {
    ($($marker:path = $path:literal,)+ #[experimental] $($emarker:path = $epath:literal,)+) => {
        fn bake_marker(marker: DataMarkerInfo, env: &databake::CrateEnv) -> databake::TokenStream {
            use databake::Bake;
            // This is a bit naughty, we need the marker's type, but we're actually
            // baking its value. This works as long as all markers are unit structs.
            if marker == icu_provider::hello_world::HelloWorldV1Marker::INFO {
                return icu_provider::hello_world::HelloWorldV1Marker.bake(env);
            }
            $(
                if marker == <$marker>::INFO {
                    return $marker.bake(env);
                }
            )+
            $(
                #[cfg(feature = "experimental_components")]
                if marker == <$emarker>::INFO {
                    return $emarker.bake(env);
                }
            )+
            unreachable!("unregistered marker {marker:?}")
        }
    }
}
crate::registry!(cb);

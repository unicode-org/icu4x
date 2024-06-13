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
//!     // HelloWorldProvider cannot provide fallback data, so we cannot deduplicate
//!     .with_locales_and_fallback([LocaleFamily::FULL], FallbackOptions::no_deduplication())
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
//! # use icu_provider::_internal::locale_core as icu_locale_core;
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
//! # use icu_provider::_internal::locale_core as icu_locale_core;
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
use std::collections::HashSet;
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
    /// By default, baked providers perform fallback internally. This field can be used to
    /// disable this behavior.
    pub use_internal_fallback: bool,
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

impl Default for Options {
    fn default() -> Self {
        Self {
            use_internal_fallback: true,
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
    use_internal_fallback: bool,
    // Temporary storage for put_payload: marker -> (bake -> {(locale, marker_attributes)})
    data: Mutex<
        HashMap<
            DataMarkerInfo,
            BTreeMap<SyncTokenStream, HashSet<(DataLocale, DataMarkerAttributes)>>,
        >,
    >,
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
            use_internal_fallback,
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
            use_internal_fallback,
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
            deps.retain(|&krate| {
                !krate.starts_with("icu_")
                    || krate == "icu_provider"
                    || krate == "icu_locale_core"
                    || krate == "icu_pattern"
            });
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
        self.data
            .lock()
            .expect("poison")
            .entry(marker)
            .or_default()
            .entry(payload)
            .or_default()
            .insert((locale.clone(), marker_attributes.clone()));
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
                pub const #singleton_ident: &'static <#marker_bake as icu_provider::DynamicDataMarker>::Yokeable = &#bake;
            }

            #maybe_msrv
            impl icu_provider::DataProvider<#marker_bake> for $provider {
                fn load(
                    &self,
                    req: icu_provider::DataRequest,
                ) -> Result<icu_provider::DataResponse<#marker_bake>, icu_provider::DataError> {
                    if req.locale.is_empty() {
                        Ok(icu_provider::DataResponse {
                            payload: icu_provider::DataPayload::from_static_ref(Self::#singleton_ident),
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
                quote!(<#marker_bake as icu_provider::DynamicDataMarker>::Yokeable),
                quote!(icu_provider::DataPayload::from_static_ref(payload)),
            )
        };

        let deduplicated_values = self
            .data
            .lock()
            .expect("poison")
            .remove(&marker)
            .unwrap_or_default();

        let maybe_msrv = maybe_msrv();

        let (load_body, iterable_body) = if deduplicated_values.is_empty() {
            (
                quote!(Err(icu_provider::DataErrorKind::MissingLocale.with_req(<#marker_bake as icu_provider::DataMarker>::INFO, req))),
                quote!(Ok(Default::default())),
            )
        } else {
            let mut structs = Vec::new();

            let values = deduplicated_values.iter().flat_map(|(bake, reqs)| {
                let bake = bake.parse::<TokenStream>().unwrap();

                let mut idents = reqs
                    .iter()
                    .map(|(locale, marker_attributes)| {
                        format!("_{}_{}", marker_attributes as &str, locale)
                            .chars()
                            .map(|ch| {
                                if ch == '-' {
                                    '_'
                                } else {
                                    ch.to_ascii_uppercase()
                                }
                            })
                            .collect::<String>()
                    })
                    .collect::<Vec<_>>();
                idents.sort();
                let ident = proc_macro2::Ident::new(&idents[0], proc_macro2::Span::call_site());

                structs.push(quote! { static #ident: #struct_type = #bake; });
                reqs.iter().map(move |la| (la.clone(), ident.clone()))
            });

            let lookup = crate::binary_search::bake(&struct_type, values);

            let load_body = if !self.use_internal_fallback
                || deduplicated_values
                    .iter()
                    .all(|(_, reqs)| reqs.iter().all(|(l, _)| l.is_und()))
            {
                quote! {
                        #(#structs)*
                        #lookup

                        if let Some(payload) = lookup(req) {
                            Ok(icu_provider::DataResponse {
                                payload: #into_data_payload,
                                metadata: Default::default(),
                            })
                        } else {
                            Err(icu_provider::DataErrorKind::MissingLocale.with_req(<#marker_bake as icu_provider::DataMarker>::INFO, req))
                        }
                }
            } else {
                self.dependencies.insert("icu_locale/compiled_data");
                quote! {
                    #(#structs)*
                    #lookup

                    let mut metadata = icu_provider::DataResponseMetadata::default();

                    let payload =  if let Some(payload) = lookup(req) {
                        payload
                    } else {
                        const FALLBACKER: icu_locale::fallback::LocaleFallbackerWithConfig<'static> =
                            icu_locale::fallback::LocaleFallbacker::new()
                                .for_config(<#marker_bake as icu_provider::DataMarker>::INFO.fallback_config);
                        let mut fallback_iterator = FALLBACKER.fallback_for(req.locale.clone());
                        loop {
                            if let Some(payload) = lookup(icu_provider::DataRequest { locale: fallback_iterator.get(), ..req }) {
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
                        payload: #into_data_payload,
                        metadata
                    })
                }
            };

            let reqs = deduplicated_values
                .values()
                .flat_map(|s| {
                    s.iter().map(|(locale, marker_attributes)| {
                        (marker_attributes.to_string(), locale.to_string())
                    })
                })
                .collect::<BTreeSet<_>>();
            let locales = reqs.iter().map(|(_, l)| l);
            let attrs = reqs.iter().map(|(a, _)| a);

            let iterable_body = if attrs.clone().all(|a| a.is_empty()) {
                // Only DataLocales
                quote!(Ok(
                    [#(icu_provider::_internal::locale_core::locale!(#locales)),*]
                    .into_iter()
                    .map(|l| (l.into(), Default::default()))
                    .collect()
                ))
            } else if locales.clone().all(|l| l == "und") {
                // Only attributes
                quote!(Ok(
                    [#(#attrs),*]
                    .into_iter()
                    .map(|a| (Default::default(), a.parse().unwrap()))
                    .collect()
                ))
            } else {
                quote!(Ok(
                    [#((#attrs, icu_provider::_internal::locale_core::locale!(#locales))),*]
                    .into_iter()
                    .map(|(a, l)| (l.into(), a.parse().unwrap()))
                    .collect()
                ))
            };

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

    fn close(&mut self) -> Result<(), DataError> {
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
        fn bake_marker(marker: DataMarkerInfo, _env: &databake::CrateEnv) -> databake::TokenStream {
            if *marker.path == *icu_provider::hello_world::HelloWorldV1Marker::INFO.path {
                return databake::quote!(icu_provider::hello_world::HelloWorldV1Marker);
            }

            $(
                if *marker.path == *$path {
                    return databake::quote!($marker);
                }
            )+

            $(
                if *marker.path == *$epath {
                    return databake::quote!($emarker);
                }
            )+

            unreachable!("unregistered marker {marker:?}")
        }
    }
}
icu_registry::registry!(cb);

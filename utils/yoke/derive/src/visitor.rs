// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Visitor for determining whether a type has type and non-static lifetime parameters
//! (except for type macros, which are largely ignored).
//!
//! Note that poor handling of type macros in `derive(Yokeable)` can only result in
//! compiler errors at worst, not unsoundness.

use std::collections::HashSet;
use syn::ext::IdentExt as _;
use syn::visit::{visit_bound_lifetimes, visit_lifetime, visit_type, visit_type_path, Visit};
use syn::{GenericParam, Ident, Lifetime, Type, TypePath};

struct Visitor<'a> {
    /// The lifetime parameter of the yokeable, stripped of any leading `r#`
    lt_param: &'a Ident,
    /// The type parameters in scope, stripped of any leading `r#`s
    typarams: &'a HashSet<Ident>,
    /// Whether we found a type parameter (or its `raw` form)
    found_typaram_usage: bool,
    /// Whether we found a usage of the `lt_param` lifetime (or its `raw` form)
    found_lt_param_usage: bool,
    /// How many underscores should be added before "yoke" in the `'__[underscores]__yoke`
    /// lifetime used by the derive.
    ///
    /// This is one more than the maximum number of underscores in
    /// (possibly raw) `'__[underscores]__yoke` lifetimes bound by `for<>` binders,
    /// or 0 if no such bound lifetimes were found.
    underscores_for_yoke_lt: usize,
}

impl<'ast> Visit<'ast> for Visitor<'_> {
    fn visit_lifetime(&mut self, lt: &'ast Lifetime) {
        if lt.ident.unraw() == *self.lt_param {
            // Note that `for<>` binders cannot introduce a lifetime already in scope,
            // so `lt.ident` necessarily refers to the lifetime parameter of the yokeable.
            self.found_lt_param_usage = true;
        }
        visit_lifetime(self, lt)
    }

    fn visit_type_path(&mut self, ty: &'ast TypePath) {
        // We only need to check ty.path.get_ident() and not ty.qself or any
        // generics in ty.path because the visitor will eventually visit those
        // types on its own
        if let Some(ident) = ty.path.get_ident() {
            if self.typarams.contains(&ident.unraw()) {
                self.found_typaram_usage = true;
            }
        }
        visit_type_path(self, ty)
    }

    fn visit_bound_lifetimes(&mut self, lts: &'ast syn::BoundLifetimes) {
        for lt in &lts.lifetimes {
            if let GenericParam::Lifetime(lt) = lt {
                let maybe_underscores_yoke = lt.lifetime.ident.unraw().to_string();

                // Check if the unraw'd lifetime ident is `__[underscores]__yoke`
                if let Some(underscores) = maybe_underscores_yoke.strip_suffix("yoke") {
                    if underscores.as_bytes().iter().all(|byte| *byte == b'_') {
                        // Since `_` is ASCII, `underscores` consists entirely of `_` characters
                        // iff it consists entirely of `b'_'` bytes, which holds iff
                        // `underscores.len()` is the number of underscores.
                        self.underscores_for_yoke_lt = self.underscores_for_yoke_lt.max(
                            // 1 more underscore, so as not to conflict
                            underscores.len() + 1,
                        );
                    }
                }
            }
        }
        visit_bound_lifetimes(self, lts);
    }
    // Type macros are ignored/skipped by default.
}

/// Checks if a type has type or parameters or uses the lifetime parameter of the yokeable type,
/// given the local context of named type parameters and the lifetime parameter.
/// Returns `(uses_type_params, uses_lifetime_param)`.
///
/// Crucially, the idents in `lt_param` and `typarams` are required to not have leading `r#`s.
pub fn check_type_for_parameters(
    lt_param: &Ident,
    typarams: &HashSet<Ident>,
    ty: &Type,
) -> (bool, bool) {
    let mut visit = Visitor {
        lt_param,
        typarams,
        found_typaram_usage: false,
        found_lt_param_usage: false,
        underscores_for_yoke_lt: 0,
    };
    visit_type(&mut visit, ty);

    (visit.found_typaram_usage, visit.found_lt_param_usage)
}

#[cfg(test)]
mod tests {
    use proc_macro2::Span;
    use std::collections::HashSet;
    use syn::{parse_quote, Ident};

    use super::check_type_for_parameters;

    fn a_ident() -> Ident {
        Ident::new("a", Span::call_site())
    }

    fn make_typarams(params: &[&str]) -> HashSet<Ident> {
        params
            .iter()
            .map(|x| Ident::new(x, Span::call_site()))
            .collect()
    }

    #[test]
    fn test_simple_type() {
        let environment = make_typarams(&["T", "U", "V"]);

        let ty = parse_quote!(Foo<'a, T>);
        let check = check_type_for_parameters(&a_ident(), &environment, &ty);
        assert_eq!(check, (true, true));

        let ty = parse_quote!(Foo<T>);
        let check = check_type_for_parameters(&a_ident(), &environment, &ty);
        assert_eq!(check, (true, false));

        let ty = parse_quote!(Foo<'static, T>);
        let check = check_type_for_parameters(&a_ident(), &environment, &ty);
        assert_eq!(check, (true, false));

        let ty = parse_quote!(Foo<'a>);
        let check = check_type_for_parameters(&a_ident(), &environment, &ty);
        assert_eq!(check, (false, true));

        let ty = parse_quote!(Foo<'a, Bar<U>, Baz<(V, u8)>>);
        let check = check_type_for_parameters(&a_ident(), &environment, &ty);
        assert_eq!(check, (true, true));

        let ty = parse_quote!(Foo<'a, W>);
        let check = check_type_for_parameters(&a_ident(), &environment, &ty);
        assert_eq!(check, (false, true));
    }

    #[test]
    fn test_assoc_types() {
        let environment = make_typarams(&["T"]);

        let ty = parse_quote!(<Foo as SomeTrait<'a, T>>::Output);
        let check = check_type_for_parameters(&a_ident(), &environment, &ty);
        assert_eq!(check, (true, true));

        let ty = parse_quote!(<Foo as SomeTrait<'static, T>>::Output);
        let check = check_type_for_parameters(&a_ident(), &environment, &ty);
        assert_eq!(check, (true, false));

        let ty = parse_quote!(<T as SomeTrait<'static, Foo>>::Output);
        let check = check_type_for_parameters(&a_ident(), &environment, &ty);
        assert_eq!(check, (true, false));
    }

    #[test]
    fn test_macro_types() {
        let environment = make_typarams(&["T"]);

        // Macro types are opaque. Note that the environment doesn't contain `U` or `V`,
        // and the `T` is basically ignored.

        let ty = parse_quote!(foo!(Foo<'a, T>));
        let check = check_type_for_parameters(&a_ident(), &environment, &ty);
        assert_eq!(check, (false, false));

        let ty = parse_quote!(foo!(Foo<T>));
        let check = check_type_for_parameters(&a_ident(), &environment, &ty);
        assert_eq!(check, (false, false));

        let ty = parse_quote!(foo!(Foo<'static, T>));
        let check = check_type_for_parameters(&a_ident(), &environment, &ty);
        assert_eq!(check, (false, false));

        let ty = parse_quote!(foo!(Foo<'a>));
        let check = check_type_for_parameters(&a_ident(), &environment, &ty);
        assert_eq!(check, (false, false));

        let ty = parse_quote!(foo!(Foo<'a, Bar<U>, Baz<(V, u8)>>));
        let check = check_type_for_parameters(&a_ident(), &environment, &ty);
        assert_eq!(check, (false, false));

        let ty = parse_quote!(foo!(Foo<'a, W>));
        let check = check_type_for_parameters(&a_ident(), &environment, &ty);
        assert_eq!(check, (false, false));
    }

    #[test]
    fn test_raw_types() {
        let environment = make_typarams(&["T", "U", "V"]);

        let ty = parse_quote!(Foo<'a, r#T>);
        let check = check_type_for_parameters(&a_ident(), &environment, &ty);
        assert_eq!(check, (true, true));

        let ty = parse_quote!(Foo<r#T>);
        let check = check_type_for_parameters(&a_ident(), &environment, &ty);
        assert_eq!(check, (true, false));

        let ty = parse_quote!(Foo<'static, r#T>);
        let check = check_type_for_parameters(&a_ident(), &environment, &ty);
        assert_eq!(check, (true, false));

        let ty = parse_quote!(Foo<'a>);
        let check = check_type_for_parameters(&a_ident(), &environment, &ty);
        assert_eq!(check, (false, true));

        let ty = parse_quote!(Foo<'a, Bar<r#U>, Baz<(r#V, u8)>>);
        let check = check_type_for_parameters(&a_ident(), &environment, &ty);
        assert_eq!(check, (true, true));

        let ty = parse_quote!(Foo<'a, r#W>);
        let check = check_type_for_parameters(&a_ident(), &environment, &ty);
        assert_eq!(check, (false, true));
    }
}

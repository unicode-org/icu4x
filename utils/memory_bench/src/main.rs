// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use quote::quote;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

fn inject_dhat_scoped_variable(syntax: &mut syn::File) {
    let dhat = syn::parse_str("let _dhat = Dhat::start_heap_profiling();")
        .expect("Unable to parse the dhat string");

    // Find the main function.
    for item in syntax.items.iter_mut() {
        if let syn::Item::Fn(ref mut fn_item) = item {
            if fn_item.sig.ident == "main" {
                // Create a new vector with the injected statement at the beginning.
                let mut new_stmts = vec![dhat];
                for stmt in fn_item.block.stmts.drain(..) {
                    new_stmts.push(stmt);
                }
                fn_item.block.stmts = new_stmts;
                return;
            }
        }
    }

    panic!("Unable to find the main function.");
}

fn inject_allocator_declaration(syntax: &mut syn::File) {
    let use_code = "use dhat::{Dhat, DhatAlloc};";
    let allocator_code = "
        #[global_allocator]
        static ALLOCATOR: DhatAlloc = DhatAlloc;
    ";
    let mut new_items = vec![
        syn::parse_str(use_code).expect("Unable to parse the dhat use string"),
        syn::parse_str(allocator_code).expect("Unable to parse the allocator string"),
    ];

    for item in syntax.items.drain(..) {
        new_items.push(item);
    }
    syntax.items = new_items;
}

/// dhat requires that the files are manually instrumented. This function automates
/// that process, by processing the AST using the syn package, modifying the AST,
/// and then finally writing it back out to a file with a slightly different name.
/// The instrumented version of the file can then be run.
fn run_dhat_injection(filename: &PathBuf) -> String {
    let mut file = File::open(&filename).expect("Unable to open file");

    let mut src = String::new();
    file.read_to_string(&mut src).expect("Unable to read file");

    let mut syntax = syn::parse_file(&src).expect("Unable to parse file");

    inject_allocator_declaration(&mut syntax);
    inject_dhat_scoped_variable(&mut syntax);

    quote!(#syntax).to_string()
}

fn main() {
    // TODO - Next commit.
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dhat_injection() {
        let path = {
            let mut path =
                PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR"));
            path.push("tests/fixtures/code.rs.txt");
            path
        };

        // The result ends up on one line of text which is a bit hard to read.
        // Split it up on the semi-colons to improve readability.
        let result: Vec<String> = run_dhat_injection(&path)
            .split(';')
            .map(String::from)
            .collect();

        assert_eq!(
            result,
            [
                // Injected code:
                "use dhat :: { Dhat , DhatAlloc } ",
                " # [global_allocator] static ALLOCATOR : DhatAlloc = DhatAlloc ",
                // Original code:
                " use std :: env ",
                " fn do_something () { println ! (\"It has another function in it.\") ",
                " println ! (\"It uses an import. {}\" , env ! (\"CARGO_PKG_VERSION\") ",
                ") ",
                // Injected code:
                " } fn main () { let _dhat = Dhat :: start_heap_profiling () ",
                //               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
                // Original:
                " println ! (\"This is a test fixture\") ",
                " }",
            ],
        );
    }
}

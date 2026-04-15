Positions on Rust Version and Style
===================================

*These positions are held by sffc@ and not necesarilly the ICU4X Technical Committee.*

## MSRV upgrades should be a last resort

In my experience, there are many, many reasons why systems use older versions of software. Some may be good, some may be bad, some may be lazy. But, ICU4X should try to "just work", regardless of the environment it is built on.

At this stage in our lifecycle, the experience of our users matters more than the experience of our maintainers. To me, MSRV upgrades that improve individual call sites are not sufficient to justify an MSRV bump.

There are of course reasons why we must sometimes bump the MSRV. Examples include:

1. Rust introduces a new feature that we plan to use widely in our architecture (example: GATs)
2. Rust introduces safe or const code that was _not possible_ to write as safe or const before
3. Rust introduces non-panicky code that was _not possible_ to _compile_ into non-panicky code before

Clarification on (2) and (3): safe and const are both code authoring constraints, whereas non-panicky is a compilation output constraint. I very much prefer using APIs that are non-panicky (such as slice operations returning `Option`), but the panics are often DCE'd out of code that uses the panicky operation. I therefore don't consider this to be justification for bumping MSRV, unless you can demonstrate that the compilation output avoids the panic. Example: slice `split_at` vs `split_at_checked`.

The TC has compromised on 4 Rust release cycles for critical fixes, 6 Rust release cycles for style fixes, and splitting utils MSRV from components MSRV. If it were up to me alone, I would prefer the line for style fixes to be more like 3-5 years.

## Write code for today's Rust constraints, but design for tomorrow's

The Rust programming language continues to evolve. There is a transparent system for tracking features as they proceed through the design and development process.

We need to write code for today's Rust. However, we should look ahead and plan for how our code will be used when Rust evolves. We can land Rust code that today is a bit "ugly" but has a path to become cleaner in a future\* Rust version.

Some might wonder how this constraint is compatible with the previous constraint. The difference is in _userland code_ vs _ICU4X code_. We can build APIs that are ergonomic to use in modern Rust even if MSRV Rust code needs workarounds.

\* By "future", I mean a feature that has a clear path forward, even if no one is currently working on it. Example: `..Default::default()` in non-exhaustive structs. If a feature has no clear path forward, then it could be considered a longer-term constraint that we should design around. Example: specialization.

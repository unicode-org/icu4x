# Why Rust for ICU4X?

The team chose Rust for writing ICU4X because it is the programming language that best fit the desired value propositions of the project.

## Good for embedded systems

ICU4X aims to serve the needs of embedded systems like smart watches and other IoT devices. The ecosystem surrounding Embedded Rust is strong for writing software to run on these devices.

Other options for embedded systems: C, C++, MicroPython, JavaScript running on [XS](https://blog.moddable.com/blog/secureprivate/).

## Good for WASM

WebAssembly does not support garbage-collected languages; the WASM GC proposal aims to address this, but there is still time before it sees broad availability. Rust also has some of the best WASM maturity because the Rust community adopted it early.

Other options for WASM: C, C++, AssemblyScript.

## Good for FFI

Rust can be compiled to virtually any platform and built into a cdylib, making it a suitable source for FFI shared libraries.

Other options for FFI: C, C++, D, Objective-C/Swift.

## Good for memory safety

Many of ICU4X's early clients, including Fuchsia and Mozilla, require memory-safe languages for all new code.

Other options for memory safety: Java, JavaScript, Python, Clojure.

## Good for performance

Clients including Mozilla require ICU4X performance to be on par with regular ICU.

Other options for performance: C, C++, Java (for ICU4J comparison).

## Good for tooling and team productivity

Rust has great tooling for tasks like code formatting, linting, and benchmarking. This makes it suitable for distributed teams since code reviews can rely on CI to check for conformance to the style guide.

Other options for good tooling and productivity: TypeScript, Java.

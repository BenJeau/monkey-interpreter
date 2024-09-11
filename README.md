# monkey-interpreter

An interpreter for the monkey programming language written in Rust without external dependencies/crates. The content of this repository is based on the book [Writing an Interpreter in Go](https://interpreterbook.com/) by Thorsten Ball, but instead of implenting it in Go, I've decided to use Rust for the following reasons:

- Rust has sum types, which remove the need of interfaces and dynamic dispatch as shown in the book
- Rust has pattern matching, which makes the code more concise and readable
- Rust can compile to WebAssembly, which makes it possible to run the interpreter in the browser (which I used to create a small demo)
- Rust has a great testing framework, which makes it easy to write unit tests
- Lastly, I love Rust 🦀

## Usage

To install and use the interpreter, you can use the following command after cloning the repository:

```bash
cargo run
```

This will start the REPL, where you can enter your code and see the output.

## WebAssembly

The interpreter can also be compiled to WebAssembly, which makes it possible to run it in the browser. A small demo is available at [https://monkey-interpreter.jeaurond.dev/](https://monkey-interpreter.jeaurond.dev/).

To compile the interpreter to WebAssembly, you need to install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) and run the following command:

```bash
wasm-pack build --target bundler
```

This will generate a `pkg` directory with the compiled WebAssembly files.

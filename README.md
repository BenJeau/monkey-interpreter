# monkey-interpreter

An interpreter for the monkey programming language written in Rust without external dependencies/crates. The content of this repository is based on the book [Writing an Interpreter in Go](https://interpreterbook.com/) by Thorsten Ball, but instead of implenting it in Go, I've decided to use Rust for the following reasons:

- Rust has sum types, which remove the need of interfaces and dynamic dispatch as shown in the book
- Rust has pattern matching, which makes the code more concise and readable
- Rust can compile to WebAssembly, which makes it possible to run the interpreter in the browser (which I used to create a small demo)
- Rust has a great testing framework, which makes it easy to write unit tests
- Lastly, I love Rust 🦀

## Usage

### WebAssembly

[![Build status](https://github.com/BenJeau/monkey-interpreter/actions/workflows/release.yaml/badge.svg?branch=main)](https://github.com/BenJeau/monkey-interpreter/actions/workflows/release.yaml)
[![NPM Version](https://img.shields.io/npm/v/@benjeau/monkey-interpreter)](https://www.npmjs.com/package/@benjeau/monkey-interpreter)

The interpreter can also be compiled to WebAssembly, which makes it possible to run it in the browser. A small demo is available at [https://monkey-interpreter.jeaurond.dev/](https://monkey-interpreter.jeaurond.dev/).

You can install the npm package `@benjeau/monkey-interpreter` by running the following command:

```bash
npm install @benjeau/monkey-interpreter
```

Then, you can use the interpreter in your TypeScript/JavaScript code like this:

```javascript
import { execute, lexer } from '@benjeau/monkey-interpreter';

const program = `
let add = fn(x, y) {
    x + y;
};

let result = add(5, 10);
`;

const tokens = lexer(program); // Returns an array of tokens
const result = execute(program); // Returns the result of the program
```

For more details, please refer to the types from within the package.

#### Compilation

To compile the interpreter to WebAssembly, you need to install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) and run the following command:

```bash
wasm-pack build --target bundler
```

This will generate a `pkg` directory with the compiled WebAssembly files.

### Rust

To install and use the interpreter, you can use the following command after cloning the repository:

```bash
cargo run
```

This will start the REPL, where you can enter your code and see the output.

## Typers
typers is a Rust project that uses WebAssembly (Wasm) to parse and solve constraints in the MiniHaskell language.

### Features
- Parse MiniHaskell code into an Abstract Syntax Tree (AST)
- Generate constraints from the AST
- Solve the generated constraints
- Render the AST and constraints in a format compatible with MathJax

### Development
To test the functionality run:
```shell
cargo run
```
This starts the ```main``` function in ```src/test.rs```.

### Build
To build the project, use the following command:
```shell
wasm-pack build --target web --out-name fmfp --out-dir ../web/wasm
```
This will compile the Rust code into a Wasm binary, which can be used in a web environment.
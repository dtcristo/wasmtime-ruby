# Fibonacci example

Basic example without using WebAssembly Interface Types, calculating Fibonacci
numbers.

## Usage

Compile Rust to WebAssembly.

```sh
cargo build --target wasm32-unknown-unknown --release
```

Install Ruby dependencies.

```sh
bundle install
```

Run example.

```sh
bundle exec ruby run.rb
```

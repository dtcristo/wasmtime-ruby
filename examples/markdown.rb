require 'wasmtime/require'
# Run `rake wasm` to compile example wasm modules first
require_relative '../wasm/markdown'

puts Markdown.render('# Hello, Ruby!')

# Note: This example is currently broken due to remove of WASM Interface Types
# support in Wasmtime. See [here](https://github.com/bytecodealliance/wasmtime/pull/1292).

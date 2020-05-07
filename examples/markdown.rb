require 'wasmtime/require'
# Run `rake wasm` to compile example wasm modules first
require_relative '../wasm/markdown'

puts Markdown.render('# Hello, Ruby!')

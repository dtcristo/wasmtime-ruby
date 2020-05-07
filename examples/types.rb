require 'wasmtime/require'
# Run `rake wasm` to compile example wasm modules first
require_relative '../wasm/types'

puts Types.add(40, 2)

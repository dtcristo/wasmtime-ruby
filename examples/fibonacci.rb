require 'wasmtime/require'
# Run `rake wasm` to compile example wasm modules first
require_relative '../wasm/fibonacci'

puts Fibonacci.fib(11)

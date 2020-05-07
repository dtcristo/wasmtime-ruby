require 'wasmtime/require'
require_relative '../wasm/fibonacci'

puts Fibonacci.fib(11)

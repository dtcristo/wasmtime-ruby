require 'wasmtime'
require_relative '../../wasm/fibonacci'

puts Fibonacci.fib(11)

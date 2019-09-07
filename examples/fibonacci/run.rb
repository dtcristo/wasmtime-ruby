require 'wasmtime'
require_relative 'target/wasm32-unknown-unknown/release/fibonacci'

puts Fibonacci.fib(5)

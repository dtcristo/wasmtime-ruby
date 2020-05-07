require 'wasmtime'

instance = Wasmtime::Instance.new(File.expand_path('../wasm/add.wat', __dir__))

puts instance.funcs[:add].call(40, 2)

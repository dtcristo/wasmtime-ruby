require 'wasmtime'

puts Wasmtime.invoke('markdown.wasm', 'render', ['# Hello, Ruby!'])

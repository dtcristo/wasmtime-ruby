require 'wasmtime'

# Load WASM module and create instance
Markdown = Wasmtime::Instance.new('markdown.wasm')

# Invoke `render` function on WASM instance
puts Markdown.invoke('render', ['# Hello, Ruby!'])

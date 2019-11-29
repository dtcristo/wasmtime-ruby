require 'wasmtime'
require_relative '../../wasm/markdown'

puts Markdown.render('# Hello, Ruby!')

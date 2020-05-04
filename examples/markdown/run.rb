require 'wasmtime/require'
require_relative '../../wasm/markdown'

puts Markdown.render('# Hello, Ruby!')

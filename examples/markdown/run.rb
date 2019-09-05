dir = File.expand_path(__dir__)
$LOAD_PATH.unshift(dir) unless $LOAD_PATH.include?(dir)

require 'wasmtime'
require 'markdown'

puts Markdown.render('# Hello, Ruby!')

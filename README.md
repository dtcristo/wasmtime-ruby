# Wasmtime for Ruby

[Wasmtime](https://wasmtime.dev/) WebAssembly runtime integration for Ruby.

## Usage

Install the `wasmtime` gem.

```sh
gem install wasmtime
```

Given an example `markdown.wasm` file is in your current directory. Require
wasmtime and the WASM module then invoke the `render` export like so.

```rb
require 'wasmtime'
require_relative 'markdown'

puts Markdown.render('# Hello, Ruby!') #=> <h1>Hello, Ruby!</h1>
```

See [examples](https://github.com/dtcristo/wasmtime-ruby/tree/master/examples/markdown)
for more usage examples.

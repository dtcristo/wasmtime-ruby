# Wasmtime for Ruby

[Wasmtime](https://wasmtime.dev/) WebAssembly runtime integration for Ruby.

## Usage

Install the `wasmtime` gem.

```sh
gem install wasmtime
```

Given a you have WASM module in your current directory, such as `markdown.wasm`
from the [examples](https://github.com/dtcristo/wasmtime-ruby/tree/master/examples/markdown).

First `require 'wasmtime'` to activate the Wasmtime require patch, allowing you
to require any `*.wasm` module as if it were a Ruby file. Doing so will
internally create a `Wasmtime::Instance` and define a Ruby module with functions
for each export.

Finally, invoke the `render` export like so.

```rb
require 'wasmtime'
require_relative 'markdown'

puts Markdown.render('# Hello, Ruby!') #=> <h1>Hello, Ruby!</h1>
```

<div align="center">
  <h1>wasmtime-ruby</h1>
  <p>
    <strong
      ><a href="https://wasmtime.dev/">Wasmtime</a> WebAssembly runtime
      integration for Ruby.</strong
    >
  </p>
</div>

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

## Development

Install Ruby dependencies.

```sh
bundle install
```

Build Rust native extension.

```sh
rake build
```

Run test suite.

```sh
rake spec
```

Format source code.

```sh
rake format
```

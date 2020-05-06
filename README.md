<div align="center">
  <h1>wasmtime-ruby</h1>
  <p>
    <strong>
      <a href="https://github.com/bytecodealliance/wasmtime">Wasmtime</a> WebAssembly runtime in Ruby
    </strong>
  </p>
  <p>
    <a href="https://rubygems.org/gems/wasmtime">
      <img src="https://img.shields.io/gem/v/wasmtime" alt="RubyGems version badge" />
    </a>
    <a href="https://github.com/dtcristo/wasmtime-ruby/actions?query=workflow%3ACI">
      <img src="https://img.shields.io/github/workflow/status/dtcristo/wasmtime-ruby/CI" alt="CI status badge" />
    </a>
  </p>
</div>

## Introduction

This project allows you to use the [Wasmtime](https://wasmtime.dev/) WebAssembly
runtime from within your Ruby project.

Why would you want that? [WebAssembly](https://webassembly.org/) (or WASM) is a
technology that allows you to write programs that run at near-native speeds in a
safe sandboxed environment. Wasmtime is a runtime that allows you to execute
WASM programs. This gem embeds Wasmtime within a native extension so you can
execute a WASM program from Ruby.

This project is pretty experimental and not production ready right now. There
are quite a few things that aren't built yet, see [TODO](#todo) section below.

**Note:** [WebAssembly Interface Types](https://github.com/webassembly/interface-types)
support has been [temporarily removed](https://github.com/bytecodealliance/wasmtime/pull/1292)
from Wasmtime. Therefore, only 32 and 64-bit integers and floats are currently
supported.

## Usage

Install the `wasmtime` gem. Compiling the native extension requires [Rust](https://rustup.rs/).

```sh
gem install wasmtime
```

Given a you have WASM module in your current directory, such as the example
`fibonacci.wasm` built from [here](https://github.com/dtcristo/wasmtime-ruby/tree/master/wasm/fibonacci).

First `require 'wasmtime/require'` to activate the Wasmtime require patch,
allowing you to require any `*.wasm` module as if it were a Ruby file. Doing so
will internally create a `Wasmtime::Instance` and define a Ruby module with
functions for each export.

Finally, invoke the `fib` export like so.

```rb
require 'wasmtime/require'
require_relative 'fibonacci'

puts Fibonacci.fib(11) #=> 89
```

If you don't like all the magic in the example above, you can do the same
without the require patch. If your project is going to be a dependency of others
use this approach too.

```rb
require 'wasmtime'

instance = Wasmtime::Instance.new('fibonacci.wasm')
puts instance.funcs[:fib].call(11) #=> 89
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

## TODO

- Add support for raw memory access and other types of exports
- Add support for imports
- Implement more of the Wasmtime API
- Add support for WASM Interface Types when they are supported in Wasmtime

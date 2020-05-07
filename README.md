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

This gem allows you to use the [Wasmtime](https://wasmtime.dev/) WebAssembly
runtime from within your Ruby project.

Why would you want that? [WebAssembly](https://webassembly.org/) (or WASM) is a
technology that allows you to write programs that run at near-native speeds in a
safe sandboxed environment. Wasmtime is a runtime for WASM programs. This gem
embedds Wasmtime in a native extension so you can now run WASM from Ruby.

This is pretty experimental and not production ready right now. There are quite
a few things that aren't built yet, see [TODO](#todo) section below.

**Note:** [WebAssembly Interface Types](https://github.com/WebAssembly/interface-types/blob/master/proposals/interface-types/Explainer.md)
support has been [temporarily removed](https://github.com/bytecodealliance/wasmtime/pull/1292)
from Wasmtime. Only 32 and 64-bit integers and floats are currently supported.

## Usage

Install the `wasmtime` gem. Pre-compiled binaries are available for
`x86_64-linux` and `x86_64-darwin-19`. Compiling the native extension requires
[Rust with rustup](https://rustup.rs/).

```sh
gem install wasmtime
```

WASM has two formats `*.wasm` (binary) and `*.wat` (human-readable text). Both
formats are supported. With the following `fibonacci.wat` file in your current
directory.

```wat
;; fibonacci.wat
(module
  (export "fib" (func $fib))
  (func $fib (param $n i32) (result i32)
    (if (i32.lt_s (get_local $n) (i32.const 2))
      (return (i32.const 1))
    )
    (return
      (i32.add
        (call $fib (i32.sub (get_local $n) (i32.const 2)))
        (call $fib (i32.sub (get_local $n) (i32.const 1)))
      )
    )
  )
)
```

In a ruby file, `require 'wasmtime/require'` to activate the Wasmtime require
patch, allowing you to require any `*.wasm` or `*.wat` module as if it were a
Ruby file. Doing so will internally create a `Wasmtime::Instance` and define a
Ruby module with functions for each export.

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

instance = Wasmtime::Instance.new('fibonacci.wat')
puts instance.funcs[:fib].call(11) #=> 89
```

## Benchmarks

None yet. But they will be impressive.

## Examples

More usage examples are provided in `examples/`. To run some of these, you first
need to compile the test WASM modules.

Install some Rust tools.

- [rustup](https://rustup.rs/) - Rust toolchain manager
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) - WASM module bundler

Install Ruby dependencies.

```sh
bundle install
```

Build the WASM modules.

```sh
bundle exec rake wasm
```

Run an example.

```sh
ruby examples/fibonacci.rb
```

## Development

Compile Rust native extension.

```sh
bundle exec rake compile
```

Run test suite.

```sh
bundle exec rake spec
```

Format source code.

```sh
bundle exec rake format
```

## TODO

- Add support for raw memory access and other types of exports
- Add support for imports
- Implement more of the Wasmtime API
- Add benchmarks for WASM program against ones in pure Ruby and true native
- Add support for WASM Interface Types when they are supported in Wasmtime

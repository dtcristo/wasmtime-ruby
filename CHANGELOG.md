# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2020-05-19

- Exported functions now live on `Wasmtime::Instance#exports` instead of `#funcs`.
- Exports hash is now keyed by strings instead of symbols.
- Added support for `Wasmtime::Memory` exports with `#data_size`, `#size` and `#grow` methods.

## [0.1.0] - 2020-05-07

### Added

- Initial release.
- Support for calling exported functions on a module.
- Support for 32 and 64-bit integers and floats in arguments and as return values.
- Require patch for defining a Ruby module with functions for each export.

[unreleased]: https://github.com/dtcristo/wasmtime-ruby/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/dtcristo/wasmtime-ruby/releases/tag/v0.2.0
[0.1.0]: https://github.com/dtcristo/wasmtime-ruby/releases/tag/v0.1.0

# frozen_string_literal: true

lib = File.expand_path('lib', __dir__)
$LOAD_PATH.unshift(lib) unless $LOAD_PATH.include?(lib)
require 'wasmtime/version'

Gem::Specification.new do |spec|
  spec.name = 'wasmtime'
  spec.version = Wasmtime::VERSION
  spec.authors = ['David Cristofaro']
  spec.email = %w[david@dtcristo.com]

  spec.summary = 'Ruby bindings for the Wasmtime WebAssembly runtime'

  spec.homepage = 'https://github.com/dtcristo/wasmtime-ruby'
  spec.required_ruby_version = '>= 2.5.0'
  spec.license = 'Apache-2.0 WITH LLVM-exception'

  spec.metadata = {
    'source_code_uri' => spec.homepage,
    'changelog_uri' =>
      'https://github.com/dtcristo/wasmtime-ruby/blob/master/CHANGELOG.md'
  }

  spec.files = Dir['{.cargo,ext,lib,src}/**/*', 'Cargo.*', 'LICENSE', '*.md']
  if ENV['NATIVE_BUNDLE']
    spec.platform = Gem::Platform::CURRENT
  else
    spec.platform = Gem::Platform::RUBY
    spec.extensions = Dir['ext/**/extconf.rb']
  end
  spec.require_paths = %w[lib]
end

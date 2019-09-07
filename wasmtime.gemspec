lib = File.expand_path('lib', __dir__)
$LOAD_PATH.unshift(lib) unless $LOAD_PATH.include?(lib)
require 'wasmtime/version'

Gem::Specification.new do |spec|
  spec.name = 'wasmtime'
  spec.version = Wasmtime::VERSION
  spec.authors = ['David Cristofaro']
  spec.email = %w[david@dtcristo.com]

  spec.summary = 'Wasmtime WebAssembly runtime integration'
  spec.homepage = 'https://github.com/dtcristo/wasmtime-ruby'
  spec.license = 'MIT'

  spec.files = Dir['{lib,ext,src}/**/*', 'Cargo.*', 'LICENSE', 'README.md']
  spec.require_paths = %w[lib]

  if ENV['NATIVE_BUNDLE']
    spec.platform = Gem::Platform::CURRENT
  else
    spec.platform = Gem::Platform::RUBY
    spec.extensions = Dir['ext/**/extconf.rb']
  end

  spec.add_development_dependency 'bundler', '~> 2.0'
  spec.add_development_dependency 'rake', '~> 12.3'
  spec.add_development_dependency 'prettier', '~> 0.15.0'
  spec.add_development_dependency 'minitest', '~> 5.11'
  spec.add_development_dependency 'minitest-reporters', '~> 1.3'
end

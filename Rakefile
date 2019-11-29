# frozen_string_literal: true

require 'bundler/setup'
require 'rspec/core/rake_task'

import 'lib/tasks/build.rake'

desc 'Build WASM for examples'
task :wasm do
  cd 'wasm/fibonacci/'
  sh 'cargo build --target wasm32-unknown-unknown --release'
  cp 'target/wasm32-unknown-unknown/release/fibonacci.wasm', '../'

  cd '../markdown/'
  sh 'env WASM_INTERFACE_TYPES=1 wasm-pack build'
  cp 'pkg/markdown.wasm', '../'
end

desc 'Format sources'
task :format do
  sh 'cargo fmt'
  sh 'bundle',
     'exec',
     'rbprettier',
     '--write',
     '**/*.{rb,rake,gemspec}',
     '**/{Rakefile,Gemfile}'
end

RSpec::Core::RakeTask.new(:spec)

task spec: :build
task default: :spec

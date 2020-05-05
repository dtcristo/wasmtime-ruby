# frozen_string_literal: true

require 'bundler/setup'
require 'rspec/core/rake_task'

import 'lib/tasks/build.rake'

desc 'Build example WASM modules'
task :wasm do
  unless `rustup target list`.include?('wasm32-unknown-unknown (installed)')
    sh 'rustup target add wasm32-unknown-unknown'
  end
  unless system('wasm-pack --version')
    p Gem::Platform.local
    if Gem::Platform.local.os == 'mswin'
      sh 'cargo install wasm-pack'
    else
      sh 'curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh'
    end
  end

  cd 'wasm/fibonacci/'
  sh 'cargo build --target wasm32-unknown-unknown --release'
  cp 'target/wasm32-unknown-unknown/release/fibonacci.wasm', '../'

  cd '../markdown/'
  ENV['WASM_INTERFACE_TYPES'] = '1'
  sh 'wasm-pack build'
  cp 'pkg/markdown.wasm', '../'

  cd '../types/'
  ENV['WASM_INTERFACE_TYPES'] = '0'
  sh 'wasm-pack build'
  # cp 'pkg/types.wasm', '../'
  cp 'pkg/types_bg.wasm', '../types.wasm'

  cd '../../'
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

task spec: %i[build wasm]
task default: :spec

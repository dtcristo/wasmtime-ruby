# frozen_string_literal: true

require 'bundler/setup'
require 'bundler/gem_tasks'
require 'rspec/core/rake_task'

import 'lib/tasks/compile.rake'

desc 'Format sources'
task :format do
  sh 'cargo fmt'
  sh 'bundle exec rbprettier --write **/*.{rb,rake,gemspec} **/{Rakefile,Gemfile}'
  sh 'git diff-index --name-only HEAD'
end

desc 'Compile test WASM modules'
task :wasm do
  unless `rustup target list`.include?('wasm32-unknown-unknown (installed)')
    sh 'rustup target add wasm32-unknown-unknown'
  end
  unless system('wasm-pack --version')
    if Gem::Platform.local.os == 'mswin'
      sh 'cargo install wasm-pack'
    else
      sh 'curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh'
    end
  end

  cd 'wasm/fibonacci/'
  sh 'cargo build --target wasm32-unknown-unknown --release'
  cp 'target/wasm32-unknown-unknown/release/fibonacci.wasm', '../'

  cd '../types/'
  sh 'wasm-pack build'
  cp 'pkg/types_bg.wasm', '../types.wasm'

  cd '../markdown/'
  ENV['WASM_INTERFACE_TYPES'] = '1'
  sh 'wasm-pack build'
  cp 'pkg/markdown.wasm', '../'

  cd '../../'
end

desc 'Verify gem installation'
task :verify do
  Bundler.with_unbundled_env do
    unless `ruby examples/add.rb` == "42\n"
      raise '`ruby examples/add.rb` failed'
    end
  end
end

RSpec::Core::RakeTask.new(:spec)

task spec: %i[compile wasm]
task default: :spec

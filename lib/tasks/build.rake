# frozen_string_literal: true

require 'rbconfig'

SOEXT = RbConfig::CONFIG['SOEXT']
DLEXT = RbConfig::CONFIG['DLEXT']

SO = File.expand_path("../../target/release/libwasmtime_ruby.#{SOEXT}", __dir__)
DL = File.expand_path("../wasmtime/native.#{DLEXT}", __dir__)

desc 'Remove build artifacts'
task :clean do
  sh 'cargo clean'
  rm_rf SO
  rm_rf DL
end

desc 'Build native extension'
task :build do
  unless `rustup target list`.include?('wasm32-unknown-unknown (installed)')
    sh 'rustup target add wasm32-unknown-unknown'
  end

  ENV['NO_LINK_RUTIE'] = 'true'
  sh 'cargo build --release'
  cp SO, DL
end

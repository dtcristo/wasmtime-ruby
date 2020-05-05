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
end

desc 'Build native extension'
task :build do
  sh 'NO_LINK_RUTIE=true cargo build --release'
  cp SO, DL
end

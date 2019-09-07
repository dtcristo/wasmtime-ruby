# frozen_string_literal: true

require 'rbconfig'

SOEXT = RbConfig::CONFIG['SOEXT']
DLEXT = RbConfig::CONFIG['DLEXT']

SO = File.expand_path("../../target/release/libwasmtime_ruby.#{SOEXT}", __dir__)
DL = File.expand_path("../wasmtime/native.#{DLEXT}", __dir__)

task :clean do
  sh 'cargo clean'
  rm_rf SO
end

task :build do
  sh 'cargo build --release'
  cp SO, DL
end

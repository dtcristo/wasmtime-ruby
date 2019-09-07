# frozen_string_literal: true

task :clean do
  sh 'cargo clean'
  rm_rf File.expand_path('../wasmtime/native.bundle', __dir__)
end

task :build do
  sh 'cargo build --release'
  cp File.expand_path('../../target/release/libwasmtime_ruby.dylib', __dir__),
     File.expand_path('../wasmtime/native.bundle', __dir__)
end

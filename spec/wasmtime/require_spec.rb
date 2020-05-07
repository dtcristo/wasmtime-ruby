# frozen_string_literal: true

require 'wasmtime/require'

RSpec.describe 'require' do
  it 'should load fibonacci module' do
    expect(require_relative('../../wasm/fibonacci')).to be_truthy
    fibonacci_lib = File.expand_path('../../wasm', __dir__)
    $LOAD_PATH.unshift(fibonacci_lib) unless $LOAD_PATH.include?(fibonacci_lib)
    expect(require('fibonacci')).to be_falsey
    expect(require('fibonacci.wasm')).to be_falsey
    expect { require('missing.wasm') }.to raise_error(LoadError)
    result = Fibonacci.fib(11)
    expect(result).to eq(89)
  end
end

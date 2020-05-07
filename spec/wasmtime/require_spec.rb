# frozen_string_literal: true

require 'wasmtime/require'

RSpec.describe 'require' do
  it 'should load a .wasm module' do
    expect(require_relative('../../wasm/fibonacci')).to be_truthy
    fibonacci_lib = File.expand_path('../../wasm', __dir__)
    $LOAD_PATH.unshift(fibonacci_lib) unless $LOAD_PATH.include?(fibonacci_lib)
    expect(require('fibonacci')).to be_falsey
    expect(require('fibonacci.wasm')).to be_falsey
    expect { require('missing.wasm') }.to raise_error(LoadError)
    result = Fibonacci.fib(11)
    expect(result).to eq(89)
  end

  it 'should load a .wat module' do
    expect(require_relative('../../wasm/add')).to be_truthy
    add_lib = File.expand_path('../../wasm', __dir__)
    $LOAD_PATH.unshift(add_lib) unless $LOAD_PATH.include?(add_lib)
    expect(require('add')).to be_falsey
    expect(require('add.wat')).to be_falsey
    expect { require('missing.wat') }.to raise_error(LoadError)
    result = Add.add(40, 2)
    expect(result).to eq(42)
  end
end

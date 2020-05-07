# frozen_string_literal: true

require 'wasmtime/require'

RSpec.describe 'require' do
  it 'should load a .wat module' do
    expect(require_relative('../../wasm/fibonacci')).to be_truthy
    path = File.expand_path('../../wasm', __dir__)
    $LOAD_PATH.unshift(path) unless $LOAD_PATH.include?(path)
    expect(require('fibonacci')).to be_falsey
    expect(require('fibonacci.wat')).to be_falsey
    expect { require('missing.wat') }.to raise_error(LoadError)
    result = Fibonacci.fib(11)
    expect(result).to eq(89)
  end

  it 'should load a .wasm module' do
    expect(require_relative('../../wasm/types')).to be_truthy
    path = File.expand_path('../../wasm', __dir__)
    $LOAD_PATH.unshift(path) unless $LOAD_PATH.include?(path)
    expect(require('types')).to be_falsey
    expect(require('types.wasm')).to be_falsey
    expect { require('missing.wasm') }.to raise_error(LoadError)
    result = Types.add(40, 2)
    expect(result).to eq(42)
  end
end

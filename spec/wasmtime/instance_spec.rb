# frozen_string_literal: true

require 'wasmtime'

describe Wasmtime::Instance do
  context 'with markdown module' do
    subject { Wasmtime::Instance.new('examples/markdown/markdown.wasm') }

    it 'will invoke render' do
      expect(subject.invoke('render', ['# Hello, Ruby!'])).to eq(
        "<h1>Hello, Ruby!</h1>\n"
      )
    end

    it 'has render export' do
      expect(subject.exports).to include('render')
    end
  end

  context 'with fibonacci module' do
    subject do
      Wasmtime::Instance.new(
        'examples/fibonacci/target/wasm32-unknown-unknown/release/fibonacci.wasm'
      )
    end

    xit 'will invoke fib' do
      expect(subject.invoke('fib', [11])).to eq(89)
    end
  end
end

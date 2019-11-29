# frozen_string_literal: true

require 'wasmtime'

describe Wasmtime::Instance do
  subject { Wasmtime::Instance.new(module_path) }

  context 'with markdown module' do
    let(:module_path) { 'wasm/markdown.wasm' }

    it 'will invoke render' do
      result = subject.invoke('render', ['# Hello, Ruby!'])
      expect(result).to eq("<h1>Hello, Ruby!</h1>\n")
    end

    it 'has render export' do
      expect(subject.exports).to include('render')
    end
  end

  context 'with fibonacci module' do
    let(:module_path) { 'wasm/fibonacci.wasm' }

    xit 'will invoke fib' do
      result = subject.invoke('fib', [11])
      expect(result).to eq(89)
    end
  end
end

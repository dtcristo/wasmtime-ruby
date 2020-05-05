# frozen_string_literal: true

require 'wasmtime'

describe Wasmtime::Instance do
  subject(:instance) { described_class.new(module_path) }

  context 'with fibonacci module' do
    let(:module_path) { 'wasm/fibonacci.wasm' }

    describe '#funcs' do
      subject(:funcs) { instance.funcs }

      it 'has fib function' do
        expect(funcs[:fib]).to be_a(Wasmtime::Func)
      end
    end
  end

  context 'with types module' do
    let(:module_path) { 'wasm/types.wasm' }

    describe '#funcs' do
      subject(:funcs) { instance.funcs }

      it 'has sum function' do
        expect(funcs[:sum]).to be_a(Wasmtime::Func)
      end
    end
  end

  xcontext 'with markdown module' do
    let(:module_path) { 'wasm/markdown.wasm' }

    describe '#funcs' do
      subject(:funcs) { instance.funcs }

      it 'has render function' do
        expect(funcs[:render]).to be_a(Wasmtime::Func)
      end
    end
  end
end

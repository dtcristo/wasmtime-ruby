# frozen_string_literal: true

require 'wasmtime'

describe Wasmtime::Instance do
  subject(:instance) { Wasmtime::Instance.new(module_path) }

  context 'with markdown module' do
    let(:module_path) { 'wasm/markdown.wasm' }

    describe '#functions' do
      subject { instance.functions }

      it 'has render function' do
        expect(subject[:render]).to be_a(Wasmtime::Function)
      end
    end
  end
end

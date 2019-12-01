# frozen_string_literal: true

require 'wasmtime'

describe Wasmtime::Instance do
  subject { Wasmtime::Instance.new(module_path) }

  context 'with markdown module' do
    let(:module_path) { 'wasm/markdown.wasm' }

    describe '#functions' do
      it 'has render function' do
        expect(subject.functions[:render]).to be_a(Wasmtime::Function)
      end
    end
  end
end

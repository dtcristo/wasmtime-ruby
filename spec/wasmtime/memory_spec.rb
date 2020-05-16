# frozen_string_literal: true

RSpec.describe Wasmtime::Memory do
  subject(:memory) { instance.exports['memory'] }
  let(:instance) { Wasmtime::Instance.new(module_path) }

  context 'with types module memory export' do
    let(:module_path) { 'wasm/types.wasm' }

    describe '#data_size' do
      subject { memory.data_size }
      it { is_expected.to be_an(Integer) }
    end

    describe '#size' do
      subject { memory.size }
      it { is_expected.to be_an(Integer) }
    end

    describe '#grow' do
      subject(:grow) { memory.grow(delta) }
      let(:delta) { 100 }

      it { is_expected.to be_an(Integer) }

      specify { expect { grow }.to change { memory.size }.by(100) }

      context 'when delta is too large' do
        let(:delta) { 10_000_000 }

        it { is_expected.to be_nil }
      end

      context 'when delta is negative' do
        let(:delta) { -100 }

        it { is_expected.to be_nil }
      end
    end
  end
end

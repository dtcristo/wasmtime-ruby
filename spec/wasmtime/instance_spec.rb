# frozen_string_literal: true

RSpec.describe Wasmtime::Instance do
  subject(:instance) { described_class.new(module_path) }

  context 'with fibonacci module' do
    let(:module_path) { 'wasm/fibonacci.wat' }

    describe '#exports' do
      subject(:exports) { instance.exports }

      it 'has fib function' do
        expect(exports['fib']).to be_a(Wasmtime::Func)
      end
    end
  end

  context 'with types module' do
    let(:module_path) { 'wasm/types.wasm' }

    describe '#exports' do
      subject(:exports) { instance.exports }

      it 'returns hash with all exports' do
        expect(exports.keys.sort).to eq(
          %w[
            __wbindgen_free
            __wbindgen_malloc
            __wbindgen_realloc
            add
            bool_bool
            f32_f32
            f64_f64
            i16_i16
            i32_i32
            i64_i64
            i8_i8
            isize_isize
            memory
            str_string
            u16_u16
            u32_u32
            u64_u64
            u8_u8
            usize_usize
            void
          ]
        )
      end

      it 'has add function' do
        expect(exports['add']).to be_a(Wasmtime::Func)
      end
    end
  end

  # context 'with markdown module' do
  #   let(:module_path) { 'wasm/markdown.wasm' }

  #   describe '#exports' do
  #     subject(:exports) { instance.exports }

  #     it 'has render function' do
  #       expect(exports['render']).to be_a(Wasmtime::Func)
  #     end
  #   end
  # end
end

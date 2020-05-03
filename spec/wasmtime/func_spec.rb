# frozen_string_literal: true

require 'wasmtime'

describe Wasmtime::Func do
  subject(:func) { instance.funcs[export] }
  let(:instance) { Wasmtime::Instance.new(module_path) }

  context 'with fibonacci module fib export' do
    let(:module_path) { 'wasm/fibonacci.wasm' }
    let(:export) { :fib }

    describe '#signature' do
      subject { func.signature }
      it { is_expected.to eq(params: %w[Integer32], result: 'Integer32') }
    end

    describe '#call' do
      subject { func.call(11) }

      it { is_expected.to eq(89) }

      it 'raises with wrong number of arguments' do
        expect { func.call(11, 12) }.to raise_error(
          ArgumentError,
          'wrong number of arguments (given 2, expected 1)'
        )
      end
    end
  end

  xcontext 'with markdown module render export' do
    let(:module_path) { 'wasm/markdown.wasm' }
    let(:export) { :render }

    describe '#signature' do
      subject { func.signature }
      it { is_expected.to eq(params: %w[String], result: 'String') }
    end

    describe '#call' do
      subject { func.call('# Hello, Ruby!') }
      it { is_expected.to eq("<h1>Hello, Ruby!</h1>\n") }
    end
  end

  xcontext 'with types module' do
    let(:module_path) { 'wasm/types.wasm' }

    context 'void export' do
      let(:export) { :void }

      describe '#signature' do
        subject { func.signature }
        it { is_expected.to eq(params: [], result: 'NilClass') }
      end

      xdescribe '#call' do
        subject { func.call }
        it { is_expected.to be_nil }
      end
    end

    xcontext 'u8_u8 export' do
      let(:export) { :u8_u8 }

      describe '#signature' do
        subject { func.signature }
        it { is_expected.to eq(params: %w[Integer32], result: 'Integer32') }
      end

      xdescribe '#call' do
        subject { func.call(8) }
        it { is_expected.to eq(9) }
      end
    end

    xcontext 'i8_i8 export' do
      let(:export) { :i8_i8 }

      describe '#signature' do
        subject { func.signature }
        it { is_expected.to eq(params: %w[Integer32], result: 'Integer32') }
      end

      xdescribe '#call' do
        subject { func.call(8) }
        it { is_expected.to eq(9) }
      end
    end

    xcontext 'u16_u16 export' do
      let(:export) { :u16_u16 }

      describe '#signature' do
        subject { func.signature }
        it { is_expected.to eq(params: %w[Integer32], result: 'Integer32') }
      end

      xdescribe '#call' do
        subject { func.call(8) }
        it { is_expected.to eq(9) }
      end
    end

    xcontext 'u32_u32 export' do
      let(:export) { :u32_u32 }

      describe '#signature' do
        subject { func.signature }
        it { is_expected.to eq(params: %w[Integer32], result: 'Integer32') }
      end

      xdescribe '#call' do
        subject { func.call(8) }
        it { is_expected.to eq(9) }
      end
    end

    xcontext 'i32_i32 export' do
      let(:export) { :i32_i32 }

      describe '#signature' do
        subject { func.signature }
        it { is_expected.to eq(params: %w[Integer32], result: 'Integer32') }
      end

      xdescribe '#call' do
        subject { func.call(8) }
        it { is_expected.to eq(9) }
      end
    end

    xcontext 'usize_usize export' do
      let(:export) { :usize_usize }

      describe '#signature' do
        subject { func.signature }
        it { is_expected.to eq(params: %w[Integer32], result: 'Integer32') }
      end

      xdescribe '#call' do
        subject { func.call(8) }
        it { is_expected.to eq(9) }
      end
    end

    xcontext 'isize_isize export' do
      let(:export) { :isize_isize }

      describe '#signature' do
        subject { func.signature }
        it { is_expected.to eq(params: %w[Integer32], result: 'Integer32') }
      end

      xdescribe '#call' do
        subject { func.call(8) }
        it { is_expected.to eq(9) }
      end
    end

    # context 'u64_u64 export' do
    #   let(:export) { :u64_u64 }

    #   describe '#signature' do
    #     subject { func.signature }
    #     it { is_expected.to eq(params: ['Integer64'], result: 'Integer64') }
    #   end

    #   describe '#call' do
    #     subject { func.call(8) }
    #     it { is_expected.to eq(9) }
    #   end
    # end

    # context 'i64_i64 export' do
    #   let(:export) { :i64_i64 }

    #   describe '#signature' do
    #     subject { func.signature }
    #     it { is_expected.to eq(params: ['Integer64'], result: 'Integer64') }
    #   end

    #   describe '#call' do
    #     subject { func.call(8) }
    #     it { is_expected.to eq(9) }
    #   end
    # end

    xcontext 'f32_f32 export' do
      let(:export) { :f32_f32 }

      describe '#signature' do
        subject { func.signature }
        it { is_expected.to eq(params: %w[Float32], result: 'Float32') }
      end

      xdescribe '#call' do
        subject { func.call(3.14159) }
        it { is_expected.to be_within(0.000001).of(6.28318) }
      end
    end

    xcontext 'f64_f64 export' do
      let(:export) { :f64_f64 }

      describe '#signature' do
        subject { func.signature }
        it { is_expected.to eq(params: %w[Float64], result: 'Float64') }
      end

      xdescribe '#call' do
        subject { func.call(3.14159) }
        it { is_expected.to eq(6.28318) }
      end
    end

    xcontext 'sum export' do
      let(:export) { :sum }

      describe '#signature' do
        subject { func.signature }
        it do
          is_expected.to eq(
            params: %w[Integer32 Integer32], result: 'Integer32'
          )
        end
      end

      xdescribe '#call' do
        subject { func.call(40, 2) }
        it { is_expected.to eq(42) }
      end
    end

    xcontext 'bool_bool export' do
      let(:export) { :bool_bool }

      describe '#signature' do
        subject { func.signature }
        it { is_expected.to eq(params: %w[Integer32], result: 'Boolean') }
      end

      xdescribe '#call' do
        subject { func.call(true) }
        it { is_expected.to be(false) }
      end
    end

    xcontext 'str_string export' do
      let(:export) { :str_string }

      describe '#signature' do
        subject { func.signature }
        it { is_expected.to eq(params: %w[String], result: 'String') }
      end

      describe '#call' do
        subject { func.call('Ruby') }
        it { is_expected.to eq('Hello, Ruby!') }
      end
    end
  end
end

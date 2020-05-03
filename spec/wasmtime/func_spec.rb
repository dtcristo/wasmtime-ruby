# frozen_string_literal: true

require 'wasmtime'

describe Wasmtime::Func do
  subject(:func) { instance.funcs[export] }
  let(:instance) { Wasmtime::Instance.new(module_path) }

  shared_examples '#signature and #call' do
    describe '#signature' do
      subject { func.signature }
      it { is_expected.to eq(expected_signature) }
    end

    describe '#call' do
      subject { func.call(*args) }
      it { is_expected.to eq(expected_result) }
    end
  end

  context 'with fibonacci module fib export' do
    let(:module_path) { 'wasm/fibonacci.wasm' }
    let(:export) { :fib }
    let(:expected_signature) { { params: %w[Integer32], result: 'Integer32' } }
    let(:args) { [11] }
    let(:expected_result) { 89 }

    include_examples '#signature and #call'

    describe '#call' do
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
    let(:expected_signature) { { params: %w[String], result: 'String' } }
    let(:args) { ['# Hello, Ruby!'] }
    let(:expected_result) { "<h1>Hello, Ruby!</h1>\n" }

    include_examples '#signature and #call'
  end

  context 'with types module' do
    let(:module_path) { 'wasm/types.wasm' }

    context 'void export' do
      let(:export) { :void }
      let(:expected_signature) { { params: [], result: 'NilClass' } }
      let(:args) { [] }
      let(:expected_result) { nil }

      include_examples '#signature and #call'
    end

    context 'u8_u8 export' do
      let(:export) { :u8_u8 }
      let(:expected_signature) do
        { params: %w[Integer32], result: 'Integer32' }
      end
      let(:args) { [8] }
      let(:expected_result) { 9 }

      include_examples '#signature and #call'
    end

    context 'i8_i8 export' do
      let(:export) { :i8_i8 }
      let(:expected_signature) do
        { params: %w[Integer32], result: 'Integer32' }
      end
      let(:args) { [8] }
      let(:expected_result) { 9 }

      include_examples '#signature and #call'
    end

    context 'u16_u16 export' do
      let(:export) { :u16_u16 }
      let(:expected_signature) do
        { params: %w[Integer32], result: 'Integer32' }
      end
      let(:args) { [8] }
      let(:expected_result) { 9 }

      include_examples '#signature and #call'
    end

    context 'u32_u32 export' do
      let(:export) { :u32_u32 }
      let(:expected_signature) do
        { params: %w[Integer32], result: 'Integer32' }
      end
      let(:args) { [8] }
      let(:expected_result) { 9 }

      include_examples '#signature and #call'
    end

    context 'i32_i32 export' do
      let(:export) { :i32_i32 }
      let(:expected_signature) do
        { params: %w[Integer32], result: 'Integer32' }
      end
      let(:args) { [8] }
      let(:expected_result) { 9 }

      include_examples '#signature and #call'
    end

    context 'usize_usize export' do
      let(:export) { :usize_usize }
      let(:expected_signature) do
        { params: %w[Integer32], result: 'Integer32' }
      end
      let(:args) { [8] }
      let(:expected_result) { 9 }

      include_examples '#signature and #call'
    end

    context 'isize_isize export' do
      let(:export) { :isize_isize }
      let(:expected_signature) do
        { params: %w[Integer32], result: 'Integer32' }
      end
      let(:args) { [8] }
      let(:expected_result) { 9 }

      include_examples '#signature and #call'
    end

    # context 'u64_u64 export' do
    #   let(:export) { :u64_u64 }
    #   let(:expected_signature) do
    #     { params: %w[Integer64], result: 'Integer64' }
    #   end
    #   let(:args) { [8] }
    #   let(:expected_result) { 9 }

    #   include_examples '#signature and #call'
    # end

    # context 'i64_i64 export' do
    #   let(:export) { :i64_i64 }
    #   let(:expected_signature) do
    #     { params: %w[Integer64], result: 'Integer64' }
    #   end
    #   let(:args) { [8] }
    #   let(:expected_result) { 9 }

    #   include_examples '#signature and #call'
    # end

    context 'f32_f32 export' do
      let(:export) { :f32_f32 }
      let(:expected_signature) { { params: %w[Float32], result: 'Float32' } }
      let(:args) { [3.14159] }
      let(:expected_result) { 6.28318 }

      include_examples '#signature and #call'
    end

    context 'f64_f64 export' do
      let(:export) { :f64_f64 }
      let(:expected_signature) { { params: %w[Float64], result: 'Float64' } }
      let(:args) { [3.14159] }
      let(:expected_result) { 6.28318 }

      include_examples '#signature and #call'
    end

    # context 'bool_bool export' do
    #   let(:export) { :bool_bool }
    #   let(:expected_signature) { { params: %w[Boolean], result: 'Boolean' } }
    #   let(:args) { [true] }
    #   let(:expected_result) { false }

    #   include_examples '#signature and #call'
    # end

    # context 'str_string export' do
    #   let(:export) { :str_string }
    #   let(:expected_signature) { { params: %w[String], result: 'String' } }
    #   let(:args) { %w[Ruby] }
    #   let(:expected_result) { 'Hello, Ruby!' }

    #   include_examples '#signature and #call'
    # end

    context 'sum export' do
      let(:export) { :sum }
      let(:expected_signature) do
        { params: %w[Integer32 Integer32], result: 'Integer32' }
      end
      let(:args) { [40, 2] }
      let(:expected_result) { 42 }

      include_examples '#signature and #call'
    end
  end
end

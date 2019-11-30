# frozen_string_literal: true

require 'wasmtime'

describe Wasmtime::Instance do
  subject { Wasmtime::Instance.new(module_path) }

  context 'with fibonacci module' do
    let(:module_path) { 'wasm/fibonacci.wasm' }

    it 'has fib export' do
      expect(subject.exports).to include('fib')
    end

    it 'will invoke fib' do
      result = subject.invoke('fib', [11])
      expect(result).to eq(89)
    end
  end

  context 'with markdown module' do
    let(:module_path) { 'wasm/markdown.wasm' }

    it 'has render export' do
      expect(subject.exports).to include('render')
    end

    it 'will invoke render' do
      result = subject.invoke('render', ['# Hello, Ruby!'])
      expect(result).to eq("<h1>Hello, Ruby!</h1>\n")
    end
  end

  context 'with types module' do
    let(:module_path) { 'wasm/types.wasm' }

    it 'has exports' do
      result = subject.exports
      expect(result).to include('void')
      expect(result).to include('u32_u32')
      expect(result).to include('i32_i32')
      expect(result).to include('f32_f32')
      expect(result).to include('f64_f64')
      expect(result).to include('bool_bool')
      expect(result).to include('str_string')
    end

    xit 'will invoke void' do
      result = subject.invoke('void')
      expect(result).to be_nil
    end

    it 'will invoke u32_u32' do
      result = subject.invoke('u32_u32', [42])
      expect(result).to eq(42)
    end

    it 'will invoke i32_i32' do
      result = subject.invoke('i32_i32', [-42])
      expect(result).to eq(-42)
    end

    xit 'will invoke f32_f32' do
      result = subject.invoke('f32_f32', [3.14159])
      expect(result).to eq(3.14159)
    end

    xit 'will invoke f64_f64' do
      result = subject.invoke('f64_f64', [3.14159])
      expect(result).to eq(3.14159)
    end

    xit 'will invoke bool_bool' do
      result = subject.invoke('bool_bool', [true])
      expect(result).to eq(false)
    end

    it 'will invoke str_string' do
      result = subject.invoke('str_string', ['Ruby'])
      expect(result).to eq('Hello, Ruby!')
    end
  end
end

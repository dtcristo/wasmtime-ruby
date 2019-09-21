# frozen_string_literal: true

require 'wasmtime'

using Wasmtime::Refinements

describe Wasmtime::Refinements do
  describe 'String' do
    describe '#camelize' do
      it 'should camelize single word' do
        expect('foo'.camelize).to eq('Foo')
      end

      it 'should camelize two words' do
        expect('fizz_buzz'.camelize).to eq('FizzBuzz')
      end

      it 'should camelize with lowecase first letter' do
        expect('camel_case'.camelize(false)).to eq('camelCase')
      end

      it 'should camelize with namespace' do
        expect('with/namespace'.camelize).to eq('With::Namespace')
      end
    end
  end
end

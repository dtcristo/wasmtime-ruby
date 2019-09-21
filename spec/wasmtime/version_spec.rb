# frozen_string_literal: true

require 'wasmtime'

describe Wasmtime do
  describe 'VERSION' do
    it 'should be defined' do
      expect(Wasmtime::VERSION).to be_truthy
    end

    it 'should be a string' do
      expect(Wasmtime::VERSION).to be_a(String)
    end
  end
end

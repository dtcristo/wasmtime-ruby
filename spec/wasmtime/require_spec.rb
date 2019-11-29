# frozen_string_literal: true

require 'wasmtime'

describe Wasmtime do
  describe 'require' do
    it 'should load markdown module' do
      expect(require_relative('../../wasm/markdown')).to be_truthy
      markdown_lib = File.expand_path('../../wasm', __dir__)
      $LOAD_PATH.unshift(markdown_lib) unless $LOAD_PATH.include?(markdown_lib)
      expect(require('markdown')).to be_falsey
      expect(require('markdown.wasm')).to be_falsey
      expect { require('missing.wasm') }.to raise_error(LoadError)
      result = Markdown.render('# Hello, Ruby!')
      expect(result).to eq("<h1>Hello, Ruby!</h1>\n")
    end
  end
end

require 'test_helper'

module Wasmtime
  class RequireTest < Minitest::Test
    def test_require_patch
      assert_equal true, require_relative('../../examples/markdown/markdown')
      markdown_lib = File.expand_path('../../examples/markdown', __dir__)
      $LOAD_PATH.unshift(markdown_lib) unless $LOAD_PATH.include?(markdown_lib)
      assert_equal false, require('markdown')
      assert_equal false, require('markdown.wasm')
      assert_raises(LoadError) { require('missing.wasm') }
      result = Markdown.render('# Hello, Ruby!')
      assert_equal "<h1>Hello, Ruby!</h1>\n", result
    end
  end
end

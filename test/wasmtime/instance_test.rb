require 'test_helper'

module Wasmtime
  class InstanceTest < Minitest::Test
    def test_invoke_with_markdown_render
      instance = Instance.new('examples/markdown/markdown.wasm')
      result = instance.invoke('render', ['# Hello, Ruby!'])
      assert_equal "<h1>Hello, Ruby!</h1>\n", result
    end
  
    def test_exports
      instance = Instance.new('examples/markdown/markdown.wasm')
      assert_includes instance.exports, 'render'
    end
  end
end

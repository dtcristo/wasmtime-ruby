# frozen_string_literal: true

require 'test_helper'

module Wasmtime
  class InstanceTest < Minitest::Test
    def test_invoke_with_markdown
      instance = Instance.new('examples/markdown/markdown.wasm')
      result = instance.invoke('render', ['# Hello, Ruby!'])
      assert_equal "<h1>Hello, Ruby!</h1>\n", result
    end

    def test_invoke_with_fibonacci
      instance =
        Instance.new(
          'examples/fibonacci/target/wasm32-unknown-unknown/release/fibonacci.wasm'
        )
      assert_equal 89, instance.invoke('fib', [11])
    end

    def test_exports_includes_function
      instance = Instance.new('examples/markdown/markdown.wasm')
      assert_includes instance.exports, 'render'
    end
  end
end

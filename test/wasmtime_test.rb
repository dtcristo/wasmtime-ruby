require 'test_helper'
require 'wasmtime'

class WasmtimeTest < Minitest::Test
  def test_it_has_a_version
    refute_nil Wasmtime::VERSION
  end

  def test_markdown
    assert_equal "<h1>Hello, Ruby!</h1>\n",
                 Wasmtime.invoke(
                   'examples/markdown/markdown.wasm',
                   'render',
                   ['# Hello, Ruby!']
                 )
  end
end

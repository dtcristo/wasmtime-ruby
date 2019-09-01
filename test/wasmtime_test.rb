require 'test_helper'
require 'wasmtime'

class WasmtimeTest < Minitest::Test
  def test_it_has_a_version
    refute_nil Wasmtime::VERSION
  end

  def test_markdown
    instance = Wasmtime::Instance.new('examples/markdown/markdown.wasm')
    result = instance.invoke('render', ['# Hello, Ruby!'])
    assert_equal "<h1>Hello, Ruby!</h1>\n", result
  end
end

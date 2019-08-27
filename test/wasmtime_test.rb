require 'test_helper'
require 'wasmtime'

class WasmtimeTest < Minitest::Test
  def test_it_has_a_version
    refute_nil Wasmtime::VERSION
  end

  def test_hello
    assert_equal('Hello from wasmtime!', Wasmtime.hello)
  end
end

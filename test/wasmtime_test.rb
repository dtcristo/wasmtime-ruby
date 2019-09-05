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

  def test_exports
    instance = Wasmtime::Instance.new('examples/markdown/markdown.wasm')
    assert_includes instance.exports, 'render'
  end

  def test_require_patch
    markdown_lib = File.expand_path('../examples/markdown', __dir__)
    $LOAD_PATH.unshift(markdown_lib) unless $LOAD_PATH.include?(markdown_lib)
    assert_equal true, require('markdown')
    assert_equal false, require('markdown.wasm')
    assert_equal false, require('markdown')
    assert_raises(LoadError) { require('missing.wasm') }
    result = Markdown.render('# Hello, Ruby!')
    assert_equal "<h1>Hello, Ruby!</h1>\n", result
  end
end

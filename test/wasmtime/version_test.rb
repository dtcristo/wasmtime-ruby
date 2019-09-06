require 'test_helper'

module Wasmtime
  class VersionTest < Minitest::Test
    def test_version_defined
      refute_nil VERSION
    end
  end
end

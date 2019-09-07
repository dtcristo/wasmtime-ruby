# frozen_string_literal: true

require 'test_helper'

using Wasmtime::Refinements

module Wasmtime
  class RefinementsTest < Minitest::Test
    def test_camelize_with_single_word
      assert_equal 'Markdown', 'markdown'.camelize
    end

    def test_camelize_with_two_words
      assert_equal 'FizzBuzz', 'fizz_buzz'.camelize
    end

    def test_camelize_with_lowercase_first_letter
      assert_equal 'camelCase', 'camel_case'.camelize(false)
    end

    def test_camelize_with_namespace
      assert_equal 'With::Namespace', 'with/namespace'.camelize
    end
  end
end

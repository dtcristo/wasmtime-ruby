# frozen_string_literal: true

require 'minitest/autorun'
require 'minitest/reporters'

require 'wasmtime'

module Minitest
  Reporters.use!(Reporters::DefaultReporter.new)
end

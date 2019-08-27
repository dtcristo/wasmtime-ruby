require 'minitest/autorun'
require 'minitest/reporters'

module Minitest
  Reporters.use!(Reporters::DefaultReporter.new)
end

# frozen_string_literal: true

require 'wasmtime'

describe Wasmtime::VERSION do
  subject { Wasmtime::VERSION }
  it { is_expected.to be_a(String) }
end

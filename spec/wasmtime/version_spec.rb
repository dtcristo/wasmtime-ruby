# frozen_string_literal: true

RSpec.describe Wasmtime::VERSION do
  subject { Wasmtime::VERSION }
  it { is_expected.to be_a(String) }
end

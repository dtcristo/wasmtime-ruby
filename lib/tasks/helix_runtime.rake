require 'helix_runtime/build_task'

class HelixRuntime::Project
  def name
    'wasmtime'
  end
end

HelixRuntime::BuildTask.new

using Wasmtime::Refinements

module Kernel
  unless defined? wasmtime_original_require
    alias_method :wasmtime_original_require, :require
    private :wasmtime_original_require
  end

  def require(path)
    wasmtime_original_require(path)
  rescue LoadError => load_error
    $LOAD_PATH.each do |load_dir|
      path = "#{path}.wasm" unless path.end_with?('.wasm')
      absolute_path = File.expand_path(path, load_dir)
      return Wasmtime.load(absolute_path) if File.file?(absolute_path)
    end
    raise load_error
  end

  unless defined? wasmtime_original_require_relative
    alias_method :wasmtime_original_require_relative, :require_relative
    private :wasmtime_original_require_relative
  end

  def require_relative(path)
    wasmtime_original_require_relative(path)
  rescue LoadError => load_error
    path = "#{path}.wasm" unless path.end_with?('.wasm')
    absolute_path = File.expand_path(path, File.dirname(caller_locations[1].path))
    return Wasmtime.load(absolute_path) if File.file?(absolute_path)
    raise load_error
  end
end

module Wasmtime
  module_function
  
  def load(absolute_path)
    return false if $LOADED_FEATURES.include?(absolute_path)
    const = absolute_path.split(File::SEPARATOR).last.delete_suffix('.wasm').camelize
    mod = Object.const_set(const, Module.new)
    instance = Wasmtime::Instance.new(absolute_path)
    instance.exports.each do |export|
      mod.define_singleton_method(export) do |*args|
        instance.invoke(export, args)
      end
    end
    $LOADED_FEATURES << absolute_path
    true
  end
end

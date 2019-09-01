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
      wasm_path = File.expand_path(path, load_dir)
      return false if $LOADED_FEATURES.include?(wasm_path)
      if File.file?(wasm_path)
        # TODO: Load WASM module here
        $LOADED_FEATURES << wasm_path
        return true
      end
    end
    raise load_error
  end
end

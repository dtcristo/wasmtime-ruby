# frozen_string_literal: true

require 'wasmtime'

using Wasmtime::Refinements

module Kernel
  unless defined?(wasmtime_original_require)
    alias_method :wasmtime_original_require, :require
    private :wasmtime_original_require
  end

  def require(path)
    wasmtime_original_require(path)
  rescue LoadError => load_error
    try_load =
      Proc.new do |path, extention|
        path_with_extention =
          path.end_with?(".#{extention}") ? path : "#{path}.#{extention}"

        if path_with_extention.start_with?('.', '/', '~')
          absolute_path = File.expand_path(path_with_extention)
          return Wasmtime.load(absolute_path) if File.file?(absolute_path)
        end
        $LOAD_PATH.each do |load_dir|
          absolute_path = File.expand_path(path_with_extention, load_dir)
          return Wasmtime.load(absolute_path) if File.file?(absolute_path)
        end
      end

    try_load.call(path, 'wasm')
    try_load.call(path, 'wat')

    raise load_error
  end

  unless defined?(wasmtime_original_require_relative)
    alias_method :wasmtime_original_require_relative, :require_relative
    private :wasmtime_original_require_relative
  end

  def require_relative(path)
    absolute_path =
      File.expand_path(path, File.dirname(caller_locations[0].absolute_path))
    require(absolute_path)
  end
end

module Wasmtime
  module_function

  def load(absolute_path)
    return false if $LOADED_FEATURES.include?(absolute_path)
    filename = absolute_path.split(File::SEPARATOR).last
    module_name =
      if filename.end_with?('.wasm')
        filename.delete_suffix('.wasm')
      else
        filename.delete_suffix('.wat')
      end
    mod = Object.const_set(module_name.camelize, Module.new)
    instance = Wasmtime::Instance.new(absolute_path)
    instance.exports.each do |name, export|
      case export
      when Wasmtime::Func
        mod.define_singleton_method(name) { |*args| export.call(*args) }
      end
    end
    $LOADED_FEATURES << absolute_path
    true
  end
end

use lazy_static::lazy_static;
use rutie::{class, methods, wrappable_struct, Module, Object};

pub struct Memory {}

impl Memory {
    pub fn new() -> Self {
        Memory {}
    }

    pub fn into_ruby(self) -> RubyMemory {
        Module::from_existing("Wasmtime")
            .get_nested_class("Memory")
            .wrap_data(self, &*MEMORY_WRAPPER)
    }
}

wrappable_struct!(Memory, MemoryWrapper, MEMORY_WRAPPER);
class!(RubyMemory);

#[rustfmt::skip]
methods!(
    RubyMemory,
    itself,
);

pub fn ruby_init() {
    Module::from_existing("Wasmtime").define(|module| {
        module.define_nested_class("Memory", None);
    });
}

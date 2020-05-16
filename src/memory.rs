use lazy_static::lazy_static;
use rutie::{class, methods, wrappable_struct, Module, Object};

pub struct Memory {}

impl Memory {
    pub fn new() -> Self {
        Memory {}
    }
}

wrappable_struct!(Memory, MemoryWrapper, MEMORY_WRAPPER);
class!(RubyMemory);

impl From<Memory> for RubyMemory {
    fn from(memory: Memory) -> Self {
        Module::from_existing("Wasmtime")
            .get_nested_class("Memory")
            .wrap_data(memory, &*MEMORY_WRAPPER)
    }
}

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

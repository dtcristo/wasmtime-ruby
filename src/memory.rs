use lazy_static::lazy_static;
use rutie::{class, methods, wrappable_struct, AnyObject, Integer, Module, NilClass, Object};
use wasmtime as w;

wrappable_struct!(w::Memory, MemoryWrapper, MEMORY_WRAPPER);
class!(RubyMemory);

impl From<w::Memory> for RubyMemory {
    fn from(memory: w::Memory) -> Self {
        Module::from_existing("Wasmtime")
            .get_nested_class("Memory")
            .wrap_data(memory, &*MEMORY_WRAPPER)
    }
}

#[rustfmt::skip]
methods!(
    RubyMemory,
    itself,

    fn ruby_instance_data_size() -> Integer {
        Integer::from(itself.get_data(&*MEMORY_WRAPPER).data_size() as u32)
    }

    fn ruby_instance_size() -> Integer {
        Integer::from(itself.get_data(&*MEMORY_WRAPPER).size())
    }

    fn ruby_instance_grow(delta: Integer) -> AnyObject {
        let memory = itself.get_data(&*MEMORY_WRAPPER);
        let delta = delta.unwrap().to_i32();
        if delta < 0 { return NilClass::new().into() }
        match memory.grow(delta as u32) {
            Ok(original) => Integer::from(original).into(),
            Err(_) => NilClass::new().into()
        }
    }
);

pub fn ruby_init() {
    Module::from_existing("Wasmtime").define(|module| {
        module.define_nested_class("Memory", None).define(|class| {
            class.def("data_size", ruby_instance_data_size);
            class.def("size", ruby_instance_size);
            class.def("grow", ruby_instance_grow);
        });
    });
}

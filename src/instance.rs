use lazy_static::lazy_static;
use rutie::{class, methods, wrappable_struct, AnyObject, Hash, Module, Object, RString};
use std::collections::HashMap;
use std::fs;
use wasmtime as w;

use crate::export::Export;
use crate::func::Func;
use crate::memory::Memory;

pub struct Instance {
    instance: w::Instance,
}

impl Instance {
    pub fn new(path: String) -> Self {
        let wasm = fs::read(path).expect("failed to read wasm file");

        let config = w::Config::new();
        // config.wasm_interface_types(true);

        let engine = w::Engine::new(&config);
        let store = w::Store::new(&engine);
        let module = w::Module::new(&store, &wasm).expect("failed to create module");
        let imports: Vec<w::Extern> = Vec::new();
        let instance = w::Instance::new(&module, &imports).expect("failed to create instance");

        Instance { instance }
    }

    fn exports(&self) -> HashMap<String, Export> {
        let mut exports = HashMap::new();

        for export in self.instance.exports() {
            match export.ty() {
                w::ExternType::Func(_) => {
                    let name = export.name().to_string();
                    let func = Func::new(export.into_func().expect("failed to create func"));
                    exports.insert(name, Export::Func(func));
                }
                w::ExternType::Memory(_) => {
                    let memory = Memory::new();
                    exports.insert(export.name().to_string(), Export::Memory(memory));
                }
                _ => {}
            }
        }

        exports
    }
}

wrappable_struct!(Instance, InstanceWrapper, INSTANCE_WRAPPER);
class!(RubyInstance);

impl From<Instance> for RubyInstance {
    fn from(instance: Instance) -> Self {
        Module::from_existing("Wasmtime")
            .get_nested_class("Instance")
            .wrap_data(instance, &*INSTANCE_WRAPPER)
    }
}

#[rustfmt::skip]
methods!(
    RubyInstance,
    itself,

    fn ruby_instance_new(path: RString) -> RubyInstance {
        Instance::new(path.expect("failed read path").to_string()).into()
    }

    fn ruby_instance_exports() -> Hash {
        let mut exports = Hash::new();

        for (export_name, export) in itself.get_data(&*INSTANCE_WRAPPER).exports().into_iter() {
            exports.store::<RString, AnyObject>(RString::new_utf8(&export_name), export.into());
        }

        exports
    }
);

pub fn ruby_init() {
    Module::from_existing("Wasmtime").define(|module| {
        module
            .define_nested_class("Instance", None)
            .define(|class| {
                class.def_self("new", ruby_instance_new);
                class.def("exports", ruby_instance_exports);
            });
    });
}

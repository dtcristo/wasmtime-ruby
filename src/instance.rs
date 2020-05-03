use lazy_static::lazy_static;
use rutie::{class, methods, wrappable_struct, Hash, Module, Object, RString, Symbol};
use std::collections::HashMap;
use std::fs;
use wasmtime as w;

use crate::function::Function;
use crate::memory::Memory;

pub struct Instance {
    instance: w::Instance,
}

impl Instance {
    pub fn new(path: String) -> Self {
        let wasm = fs::read(path).unwrap();

        let config = w::Config::new();
        // config.wasm_interface_types(true);

        let engine = w::Engine::new(&config);
        let store = w::Store::new(&engine);
        let module = w::Module::new(&store, &wasm).unwrap();
        let imports: Vec<w::Extern> = Vec::new();
        let instance = w::Instance::new(&module, &imports).unwrap();

        Instance { instance }
    }

    fn exports(&self) -> (HashMap<String, Function>, HashMap<String, Memory>) {
        let mut functions = HashMap::new();
        let mut memories = HashMap::new();

        for export in self.instance.exports() {
            match export.ty() {
                w::ExternType::Func(_) => {
                    let name = export.name().to_string();
                    let function = Function::new(export.into_func().unwrap());
                    functions.insert(name, function);
                }
                w::ExternType::Memory(_) => {
                    let memory = Memory::new();
                    memories.insert(export.name().to_string(), memory);
                }
                _ => {}
            }
        }

        (functions, memories)
    }

    pub fn functions(&self) -> HashMap<String, Function> {
        let (functions, _) = self.exports();
        functions
    }

    pub fn into_ruby(self) -> RubyInstance {
        Module::from_existing("Wasmtime")
            .get_nested_class("Instance")
            .wrap_data(self, &*INSTANCE_WRAPPER)
    }
}

wrappable_struct!(Instance, InstanceWrapper, INSTANCE_WRAPPER);
class!(RubyInstance);

#[rustfmt::skip]
methods!(
    RubyInstance,
    itself,

    fn ruby_instance_new(path: RString) -> RubyInstance {
        Instance::new(path.unwrap().to_string()).into_ruby()
    }

    fn ruby_instance_functions() -> Hash {
        let mut functions = Hash::new();
        for (export_name, function) in itself.get_data(&*INSTANCE_WRAPPER).functions().into_iter() {
            functions.store(Symbol::new(&export_name), function.into_ruby());
        }
        functions
    }
);

pub fn ruby_init() {
    Module::from_existing("Wasmtime").define(|module| {
        module
            .define_nested_class("Instance", None)
            .define(|class| {
                class.def_self("new", ruby_instance_new);
                class.def("functions", ruby_instance_functions);
            });
    });
}

use lazy_static::lazy_static;
use rutie::{class, methods, wrappable_struct, Hash, Module, Object, RString, Symbol};
use std::collections::HashMap;
use std::fs;
use wasmtime as w;

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

    fn exports(&self) -> (HashMap<String, Func>, HashMap<String, Memory>) {
        let mut funcs = HashMap::new();
        let mut memories = HashMap::new();

        for export in self.instance.exports() {
            match export.ty() {
                w::ExternType::Func(_) => {
                    let name = export.name().to_string();
                    let func = Func::new(export.into_func().expect("failed to create func"));
                    funcs.insert(name, func);
                }
                w::ExternType::Memory(_) => {
                    let memory = Memory::new();
                    memories.insert(export.name().to_string(), memory);
                }
                _ => {}
            }
        }

        (funcs, memories)
    }

    pub fn funcs(&self) -> HashMap<String, Func> {
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
        Instance::new(path.expect("failed read path").to_string()).into_ruby()
    }

    fn ruby_instance_funcs() -> Hash {
        let mut funcs = Hash::new();
        for (export_name, func) in itself.get_data(&*INSTANCE_WRAPPER).funcs().into_iter() {
            funcs.store(Symbol::new(&export_name), func.into_ruby());
        }
        funcs
    }
);

pub fn ruby_init() {
    Module::from_existing("Wasmtime").define(|module| {
        module
            .define_nested_class("Instance", None)
            .define(|class| {
                class.def_self("new", ruby_instance_new);
                class.def("funcs", ruby_instance_funcs);
            });
    });
}

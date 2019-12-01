use lazy_static::lazy_static;
use rutie::{class, methods, wrappable_struct, Hash, Module, Object, RString, Symbol};
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;
use wasmtime as w;
use wasmtime_interface_types as wit;

use crate::function::Function;
use crate::memory::Memory;

pub struct Instance {
    instance: w::HostRef<w::Instance>,
    module_data: Rc<wit::ModuleData>,
}

impl Instance {
    pub fn new(path: String) -> Self {
        let wasm = fs::read(path).unwrap();

        let config = w::Config::new();
        let engine = w::HostRef::new(w::Engine::new(&config));
        let store = w::HostRef::new(w::Store::new(&engine));
        let module = w::HostRef::new(w::Module::new(&store, &wasm).unwrap());
        let imports: Vec<w::Extern> = Vec::new();
        let instance = w::HostRef::new(w::Instance::new(&store, &module, &imports).unwrap());

        let module_data = Rc::new(wit::ModuleData::new(&wasm).unwrap());

        Instance {
            instance,
            module_data,
        }
    }

    fn exports(&self) -> (HashMap<String, Function>, HashMap<String, Memory>) {
        let mut functions = HashMap::new();
        let mut memories = HashMap::new();

        for export in self.instance.borrow().module().borrow().exports().iter() {
            match export.r#type() {
                w::ExternType::ExternFunc(_) => {
                    let function = Function::new(
                        self.instance.clone(),
                        self.module_data.clone(),
                        export.name().to_string(),
                    );
                    functions.insert(export.name().to_string(), function);
                }
                w::ExternType::ExternMemory(_) => {
                    let memory = Memory::new(export.name().to_string());
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

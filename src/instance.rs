use lazy_static::lazy_static;
use rutie::{class, methods, wrappable_struct, Hash, Module, Object, RString, Symbol};
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;
use wasm_webidl_bindings as wwb;
use wasmtime as w;
use wasmtime_interface_types as wit;

use crate::function::Function;
use crate::memory::Memory;

pub struct Instance {
    instance: Rc<w::Instance>,
    module_data: Rc<wit::ModuleData>,
}

impl Instance {
    pub fn new(path: String) -> Self {
        let wasm = fs::read(path.clone()).unwrap();
        // let mut config = walrus::ModuleConfig::default();
        // config.on_parse(move |module, indices_to_ids| {
        //     let wasm2 = fs::read(path.clone()).unwrap();
        //     let webidl_bindings = wwb::binary::decode(indices_to_ids, &wasm2)?;
        //     println!("The parsed Web IDL bindings are {:#?}", webidl_bindings);
        //     // Insert the `webidl_bindings` into the module as a custom section.
        //     module.customs.add(webidl_bindings);
        //     Ok(())
        // });
        // config.parse(&wasm).unwrap();

        // let config = w::Config::new();
        // let engine = w::Engine::new(&config);
        // let store = w::Store::new(&engine);
        let store = w::Store::default();
        let module = w::Module::new(&store, &wasm).unwrap();
        let imports: Vec<w::Extern> = Vec::new();
        let instance = Rc::new(w::Instance::new(&module, &imports).unwrap());

        let module_data = Rc::new(wit::ModuleData::new(&wasm).unwrap());

        Instance {
            instance,
            module_data,
        }
    }

    fn exports(&self) -> (HashMap<String, Function>, HashMap<String, Memory>) {
        let mut functions = HashMap::new();
        let mut memories = HashMap::new();

        for export in self.instance.module().exports().iter() {
            match export.ty() {
                w::ExternType::Func(_) => {
                    let function = Function::new(
                        self.instance.clone(),
                        self.module_data.clone(),
                        export.name().to_string(),
                    );
                    functions.insert(export.name().to_string(), function);
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

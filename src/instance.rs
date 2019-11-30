use lazy_static::lazy_static;
use rutie::{
    class, methods, wrappable_struct, AnyObject, Array, Hash, Module, NilClass, Object, RString,
    Symbol,
};
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;
use wasmtime as w;
use wasmtime_interface_types as wit;

use crate::function::Function;
use crate::memory::Memory;
use crate::wasm_value::WasmValue;

pub struct Instance {
    instance: w::HostRef<w::Instance>,
    module_data: Rc<wit::ModuleData>,
    functions: HashMap<String, Function>,
    memories: HashMap<String, Memory>,
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

        let (functions, memories) = parse_exports(instance.clone(), module_data.clone());

        Instance {
            instance,
            module_data,
            functions,
            memories,
        }
    }

    // TODO: Delete this
    pub fn exports(&mut self) -> Vec<String> {
        let exports: Vec<String> = self
            .instance
            .borrow()
            .module()
            .borrow()
            .exports()
            .iter()
            .filter_map(|e| match e.r#type() {
                w::ExternType::ExternFunc(_) => Some(e.name().to_string()),
                _ => None,
            })
            .collect();

        exports
    }

    pub fn invoke(&mut self, export: &str, args: &[WasmValue]) -> Vec<WasmValue> {
        let args_native: Vec<wit::Value> = args.iter().map(|wv| wv.clone().into()).collect();
        self.module_data
            .invoke_export(&mut self.instance, export, &args_native)
            .expect("unable to invoke export")
            .into_iter()
            .map(|v| v.into())
            .collect()
    }
}

fn parse_exports(
    instance: w::HostRef<w::Instance>,
    module_data: Rc<wit::ModuleData>,
) -> (HashMap<String, Function>, HashMap<String, Memory>) {
    let mut functions = HashMap::new();
    let mut memories = HashMap::new();

    instance
        .borrow()
        .module()
        .borrow()
        .exports()
        .iter()
        .for_each(|export| match export.r#type() {
            w::ExternType::ExternFunc(_) => {
                let function = Function::new(
                    instance.clone(),
                    module_data.clone(),
                    export.name().to_string(),
                );
                functions.insert(export.name().to_string(), function);
            }
            w::ExternType::ExternMemory(_) => {
                let memory = Memory::new(export.name().to_string());
                memories.insert(export.name().to_string(), memory);
            }
            _ => {}
        });

    (functions, memories)
}

wrappable_struct!(Instance, InstanceWrapper, INSTANCE_WRAPPER);
class!(RubyInstance);

#[rustfmt::skip]
methods!(
    RubyInstance,
    itself,

    fn ruby_instance_new(path: RString) -> RubyInstance {
        let instance = Instance::new(path.unwrap().to_string());
        Module::from_existing("Wasmtime")
            .get_nested_class("Instance")
            .wrap_data(instance, &*INSTANCE_WRAPPER)
    }

    fn ruby_instance_exports() -> Array {
        let instance = itself.get_data_mut(&*INSTANCE_WRAPPER);
        let mut exports = Array::new();
        instance.exports().iter().for_each(|export| {
            exports.push(RString::new_utf8(export));
        });
        exports
    }

    fn ruby_instance_functions() -> NilClass {
        let instance = itself.get_data_mut(&*INSTANCE_WRAPPER);

        instance.functions.iter().for_each(|(export_name, function)| {
            println!("{}: {:?} -> {:?}", export_name, &function.param_types, &function.result_types);
        });

        // let mut functions = Hash::new();
        // instance.exports().iter().for_each(|export| {
        //     let mut function = Hash::new();
        //     let mut params = Array::new();
        //     let decoded_params =
        //     functions.store(Symbol::new("params"), params);
        //     let mut result = Array::new();
        //     if results.len() == 1 {
        //         results.into_iter().next().unwrap().into()
        //     } else {
        //         let mut results_array = Array::new();
        //         for result in results.into_iter() {
        //             let object: AnyObject = result.into();
        //             results_array.push(object);
        //         }
        //         results_array.into()
        //     }
        //     functions.store(Symbol::new("result"), function);

        //     functions.store(Symbol::new(export), function);
        // });
        // exports
        NilClass::new()
    }

    fn ruby_instance_invoke(export: RString, args: Array) -> AnyObject {
        let export = export.unwrap().to_string();
        let args: Vec<WasmValue> = args.unwrap().into_iter().map(|o| o.into()).collect();
        let instance = itself.get_data_mut(&*INSTANCE_WRAPPER);
        let results = instance.invoke(&export, &args[..]);
        if results.len() == 1 {
            results.into_iter().next().unwrap().into()
        } else {
            let mut results_array = Array::new();
            for result in results.into_iter() {
                let object: AnyObject = result.into();
                results_array.push(object);
            }
            results_array.into()
        }
    }
);

pub fn ruby_init() {
    Module::from_existing("Wasmtime").define(|module| {
        module
            .define_nested_class("Instance", None)
            .define(|class| {
                class.def_self("new", ruby_instance_new);
                class.def("exports", ruby_instance_exports);
                class.def("functions", ruby_instance_functions);
                class.def("invoke", ruby_instance_invoke);
            });
    });
}

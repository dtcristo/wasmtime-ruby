use std::fs;
use wasmtime as w;
use wasmtime_interface_types as wit;

use crate::wasm_value::WasmValue;

pub struct Instance {
    instance: w::HostRef<w::Instance>,
    module_data: wit::ModuleData,
}

impl Instance {
    pub fn new(path: String) -> Instance {
        let wasm = fs::read(path).unwrap();

        let config = w::Config::new();
        let engine = w::HostRef::new(w::Engine::new(&config));
        let store = w::HostRef::new(w::Store::new(&engine));
        let module = w::HostRef::new(w::Module::new(&store, &wasm).unwrap());
        let imports: Vec<w::Extern> = Vec::new();
        let instance = w::HostRef::new(w::Instance::new(&store, &module, &imports).unwrap());

        let module_data = wit::ModuleData::new(&wasm).unwrap();

        Instance {
            instance,
            module_data,
        }
    }

    pub fn exports(&mut self) -> Vec<String> {
        self.instance
            .borrow()
            .module()
            .borrow()
            .exports()
            .iter()
            .filter_map(|e| match e.r#type() {
                w::ExternType::ExternFunc(_) => Some(e.name().to_string()),
                _ => None,
            })
            .collect()
    }

    pub fn invoke(&mut self, export: &str, args: &[WasmValue]) -> Vec<WasmValue> {
        let args_native: Vec<wit::Value> = args.iter().map(|wv| wv.clone().into()).collect();
        self.module_data
            .invoke_export(&mut self.instance, export, &args_native)
            .expect("Unable to invoke export")
            .into_iter()
            .map(|v| v.into())
            .collect()
    }
}

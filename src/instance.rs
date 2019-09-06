use std::fs;
use wasmtime_environ::Export;
use wasmtime_interface_types::{ModuleData, Value};
use wasmtime_jit::{Context, InstanceHandle};

use crate::wasm_value::WasmValue;

pub struct Instance {
    cx: Context,
    handle: InstanceHandle,
    data: ModuleData,
}

impl Instance {
    pub fn new(path: String) -> Instance {
        let bytes = fs::read(path).unwrap();
        let isa = {
            let isa_builder = cranelift_native::builder().unwrap();
            let flag_builder = cranelift_codegen::settings::builder();
            isa_builder.finish(cranelift_codegen::settings::Flags::new(flag_builder))
        };
        let mut cx = Context::with_isa(isa);
        let data = ModuleData::new(&bytes).unwrap();
        let handle = cx.instantiate_module(None, &bytes).unwrap();
        Instance { cx, handle, data }
    }

    pub fn exports(&mut self) -> Vec<String> {
        self.handle
            .exports()
            .filter_map(|(name, export)| match export {
                Export::Function(_) => Some(name.to_string()),
                _ => None,
            })
            .collect()
    }

    pub fn invoke(&mut self, export: &str, args: &[WasmValue]) -> Vec<WasmValue> {
        let args_native: Vec<Value> = args.iter().map(|wv| wv.clone().into()).collect();
        self.data
            .invoke(&mut self.cx, &mut self.handle, export, &args_native)
            .expect("Unable to invoke export")
            .into_iter()
            .map(|v| v.into())
            .collect()
    }
}

use helix::sys::VALUE;
use helix::*;
use std::convert::TryInto;
use std::fs::read;
use wasmtime_interface_types::{ModuleData, Value};
use wasmtime_jit::{Context, InstanceHandle};

#[derive(Clone, Debug)]
pub enum WasmValue {
    Integer(i64),
    Float(f64),
    String(String),
}

pub enum CheckedWasmValue {
    Integer(<i64 as FromRuby>::Checked),
    Float(f64),
    String(<String as FromRuby>::Checked),
}

impl FromRuby for WasmValue {
    type Checked = CheckedWasmValue;

    fn from_ruby(value: VALUE) -> CheckResult<CheckedWasmValue> {
        if let Ok(checked) = i64::from_ruby(value) {
            Ok(CheckedWasmValue::Integer(checked))
        } else if let Ok(checked) = f64::from_ruby(value) {
            let float = f64::from_checked(checked);
            if float.is_normal() {
                Ok(CheckedWasmValue::Float(float))
            } else {
                type_error!(format!("Cannot convert {} into a WASM number", float))
            }
        } else if let Ok(checked) = String::from_ruby(value) {
            Ok(CheckedWasmValue::String(checked))
        } else {
            type_error!(value, "a WASM value")
        }
    }

    fn from_checked(checked: CheckedWasmValue) -> WasmValue {
        match checked {
            CheckedWasmValue::Integer(c) => WasmValue::Integer(FromRuby::from_checked(c)),
            CheckedWasmValue::Float(c) => WasmValue::Float(c),
            CheckedWasmValue::String(c) => WasmValue::String(FromRuby::from_checked(c)),
        }
    }
}

impl ToRuby for WasmValue {
    fn to_ruby(self) -> ToRubyResult {
        match self {
            WasmValue::Integer(v) => v.to_ruby(),
            WasmValue::Float(v) => v.to_ruby(),
            WasmValue::String(v) => v.to_ruby(),
        }
    }
}

impl Into<Value> for WasmValue {
    fn into(self) -> Value {
        match self {
            WasmValue::Integer(v) => v.into(),
            WasmValue::Float(v) => v.into(),
            WasmValue::String(v) => v.into(),
        }
    }
}

impl From<Value> for WasmValue {
    fn from(value: Value) -> WasmValue {
        match value {
            Value::String(v) => WasmValue::String(v),
            Value::I32(v) => WasmValue::Integer(v.into()),
            Value::U32(v) => WasmValue::Integer(v.into()),
            Value::I64(v) => WasmValue::Integer(v),
            Value::U64(v) => WasmValue::Integer(v.try_into().unwrap()),
            Value::F32(v) => WasmValue::Float(v.into()),
            Value::F64(v) => WasmValue::Float(v),
        }
    }
}

struct Module {
    cx: Context,
    handle: InstanceHandle,
    data: ModuleData,
}

impl Module {
    fn invoke(&mut self, export: &str, args: &[Value]) -> Vec<Value> {
        self.data
            .invoke(&mut self.cx, &mut self.handle, export, args)
            .expect("Unable to invoke export")
    }
}

ruby! {
    class Wasmtime {
        def invoke(path: String, export: String, args: Vec<WasmValue>) -> WasmValue {
            let mut module = load_file(path);
            let value_args: Vec<Value> = args.iter().map(|v| v.clone().into()).collect();
            module.invoke(&export, &value_args).first().unwrap().to_owned().into()
        }
    }
}

fn load_file(path: String) -> Module {
    let bytes = read(path).unwrap();

    let isa = {
        let isa_builder = cranelift_native::builder().unwrap();
        let flag_builder = cranelift_codegen::settings::builder();
        isa_builder.finish(cranelift_codegen::settings::Flags::new(flag_builder))
    };

    let mut cx = Context::with_isa(isa);
    let data = ModuleData::new(&bytes).unwrap();
    let handle = cx.instantiate_module(None, &bytes).unwrap();

    Module { cx, handle, data }
}

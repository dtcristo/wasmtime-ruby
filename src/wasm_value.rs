use rutie::rubysys::value::ValueType;
use rutie::{AnyObject, Fixnum, Float, Object, RString};
use std::convert::TryInto;
use wasmtime_interface_types::Value;

#[derive(Clone, Debug)]
pub struct WasmValue(pub Value);

impl From<Value> for WasmValue {
    fn from(value: Value) -> Self {
        WasmValue(value)
    }
}

impl Into<Value> for WasmValue {
    fn into(self) -> Value {
        self.0
    }
}

impl From<AnyObject> for WasmValue {
    fn from(object: AnyObject) -> Self {
        match object.ty() {
            ValueType::RString => {
                Value::String(object.try_convert_to::<RString>().unwrap().to_string()).into()
            }
            ValueType::Fixnum => {
                Value::I32(object.try_convert_to::<Fixnum>().unwrap().to_i32()).into()
            }
            ValueType::Float => {
                Value::F64(object.try_convert_to::<Float>().unwrap().to_f64()).into()
            }
            _ => panic!("Unable to convert object to WasmValue"),
        }
    }
}

impl Into<AnyObject> for WasmValue {
    fn into(self) -> AnyObject {
        let value = self.0;
        match value {
            Value::String(v) => RString::new_utf8(&v).into(),
            Value::I32(v) => Fixnum::new(v.into()).into(),
            Value::U32(v) => Fixnum::new(v.try_into().unwrap()).into(),
            Value::I64(v) => Fixnum::new(v).into(),
            Value::U64(v) => Fixnum::new(v.try_into().unwrap()).into(),
            Value::F32(v) => Float::new(v.into()).into(),
            Value::F64(v) => Float::new(v).into(),
        }
    }
}

use rutie::rubysys::value::ValueType;
use rutie::{AnyObject, Fixnum, Float, Object, RString};
use std::convert::TryInto;
use wasmtime_interface_types as wit;

#[derive(Clone, Debug)]
pub struct WasmValue(pub wit::Value);

impl From<wit::Value> for WasmValue {
    fn from(value: wit::Value) -> Self {
        WasmValue(value)
    }
}

impl Into<wit::Value> for WasmValue {
    fn into(self) -> wit::Value {
        self.0
    }
}

impl From<AnyObject> for WasmValue {
    fn from(object: AnyObject) -> Self {
        match object.ty() {
            ValueType::RString => {
                wit::Value::String(object.try_convert_to::<RString>().unwrap().to_string()).into()
            }
            ValueType::Fixnum => {
                wit::Value::I32(object.try_convert_to::<Fixnum>().unwrap().to_i32()).into()
            }
            ValueType::Float => {
                wit::Value::F64(object.try_convert_to::<Float>().unwrap().to_f64()).into()
            }
            _ => panic!("Unable to convert object to WasmValue"),
        }
    }
}

impl Into<AnyObject> for WasmValue {
    fn into(self) -> AnyObject {
        let value = self.0;
        match value {
            wit::Value::String(v) => RString::new_utf8(&v).into(),
            wit::Value::I32(v) => Fixnum::new(v.into()).into(),
            wit::Value::U32(v) => Fixnum::new(v.try_into().unwrap()).into(),
            wit::Value::I64(v) => Fixnum::new(v).into(),
            wit::Value::U64(v) => Fixnum::new(v.try_into().unwrap()).into(),
            wit::Value::F32(v) => Float::new(v.into()).into(),
            wit::Value::F64(v) => Float::new(v).into(),
        }
    }
}

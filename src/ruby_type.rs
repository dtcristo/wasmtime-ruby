use rutie::{AnyObject, RString};
use wasmtime as w;

#[derive(Debug, Copy, Clone)]
pub enum RubyType {
    Integer32,
    Integer64,
    Float32,
    Float64,
    // String,
    // Boolean,
    NilClass,
    Unsupported,
}

impl From<RubyType> for AnyObject {
    fn from(ruby_type: RubyType) -> AnyObject {
        RString::new_utf8(&format!("{:?}", ruby_type)).into()
    }
}

impl From<&w::ValType> for RubyType {
    fn from(val_type: &w::ValType) -> Self {
        match val_type {
            w::ValType::I32 => RubyType::Integer32,
            w::ValType::I64 => RubyType::Integer64,
            w::ValType::F32 => RubyType::Float32,
            w::ValType::F64 => RubyType::Float64,
            _ => RubyType::Unsupported,
        }
    }
}

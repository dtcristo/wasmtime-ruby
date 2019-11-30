use std::rc::Rc;
use wasm_webidl_bindings::ast;
use wasmtime as w;
use wasmtime_interface_types as wit;

pub struct Function {
    instance: w::HostRef<w::Instance>,
    module_data: Rc<wit::ModuleData>,
    pub export_name: String,
    pub param_types: Vec<RubyType>,
    pub result_types: Vec<RubyType>,
}

impl Function {
    pub fn new(
        instance: w::HostRef<w::Instance>,
        module_data: Rc<wit::ModuleData>,
        export_name: String,
    ) -> Self {
        let mut handle = instance.borrow().handle().clone();
        let export_binding = module_data
            .binding_for_export(&mut handle, &export_name)
            .unwrap();
        let params = export_binding.param_bindings().unwrap();
        let results = export_binding.result_bindings().unwrap();
        let param_types = decode_params(&params);
        let result_types = decode_results(&results);

        Function {
            instance,
            module_data,
            export_name,
            param_types,
            result_types,
        }
    }
}

#[derive(Debug)]
pub enum RubyType {
    Integer32,
    Integer64,
    Float32,
    Float64,
    String,
    Boolean,
    Unsupported,
}

fn decode_params(params: &[ast::IncomingBindingExpression]) -> Vec<RubyType> {
    params
        .iter()
        .map(|expr| match expr {
            ast::IncomingBindingExpression::As(e) => match e.ty {
                walrus::ValType::I32 => RubyType::Integer32,
                walrus::ValType::I64 => RubyType::Integer64,
                walrus::ValType::F32 => RubyType::Float32,
                walrus::ValType::F64 => RubyType::Float64,
                walrus::ValType::V128 | walrus::ValType::Anyref => RubyType::Unsupported,
            },
            ast::IncomingBindingExpression::AllocUtf8Str(_) => RubyType::String,
            _ => RubyType::Unsupported,
        })
        .collect()
}

fn decode_results(results: &[ast::OutgoingBindingExpression]) -> Vec<RubyType> {
    results
        .iter()
        .map(|expr| match expr {
            ast::OutgoingBindingExpression::As(e) => {
                match e.ty {
                    ast::WebidlTypeRef::Scalar(s) => match s {
                        ast::WebidlScalarType::Byte
                        | ast::WebidlScalarType::Octet
                        | ast::WebidlScalarType::Short
                        | ast::WebidlScalarType::UnsignedShort
                        | ast::WebidlScalarType::Long
                        | ast::WebidlScalarType::UnsignedLong => RubyType::Integer32,
                        ast::WebidlScalarType::LongLong
                        | ast::WebidlScalarType::UnsignedLongLong => RubyType::Integer64,
                        ast::WebidlScalarType::Float | ast::WebidlScalarType::UnrestrictedFloat => {
                            RubyType::Float32
                        }
                        ast::WebidlScalarType::Double
                        | ast::WebidlScalarType::UnrestrictedDouble => RubyType::Float64,
                        ast::WebidlScalarType::Boolean => RubyType::Boolean,
                        _ => panic!("failed to decode results, unsupported type: ({:?})", s),
                    },
                    _ => panic!("failed to decode results, unsupported type: {:?}", e.ty),
                }
            }
            ast::OutgoingBindingExpression::Utf8Str(_) => RubyType::String,
            _ => panic!("failed to decode results, unsupported type: {:?}", expr),
        })
        .collect()
}

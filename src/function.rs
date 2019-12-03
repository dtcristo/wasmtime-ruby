use lazy_static::lazy_static;
use rutie as r;
use rutie::rubysys;
use rutie::{
    class, methods, wrappable_struct, AnyObject, Array, Float, Hash, Integer, Module, NilClass,
    Object, RString, Symbol,
};
use std::convert::TryInto;
use std::mem;
use std::rc::Rc;
use wasm_webidl_bindings::ast;
use wasmtime as w;
use wasmtime_interface_types as wit;

pub struct Function {
    instance: w::HostRef<w::Instance>,
    module_data: Rc<wit::ModuleData>,
    export_name: String,
    param_types: Vec<RubyType>,
    result_type: RubyType,
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
        let param_types = parse_param_types(&params);
        let result_type = parse_result_type(&results);

        Function {
            instance,
            module_data,
            export_name,
            param_types,
            result_type,
        }
    }

    pub fn call(&mut self, args: &[wit::Value]) -> Vec<wit::Value> {
        self.module_data
            .invoke_export(&mut self.instance, &self.export_name, args)
            .expect("failed to invoke export")
    }

    pub fn into_ruby(self) -> RubyFunction {
        Module::from_existing("Wasmtime")
            .get_nested_class("Function")
            .wrap_data(self, &*FUNCTION_WRAPPER)
    }
}

#[derive(Debug, Copy, Clone)]
enum RubyType {
    Integer32,
    Integer64,
    Float32,
    Float64,
    String,
    Boolean,
    NilClass,
    Unsupported,
}

impl Into<AnyObject> for RubyType {
    fn into(self) -> AnyObject {
        RString::new_utf8(&format!("{:?}", self)).into()
    }
}

fn parse_param_types(params: &[ast::IncomingBindingExpression]) -> Vec<RubyType> {
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

fn parse_result_type(results: &[ast::OutgoingBindingExpression]) -> RubyType {
    match results.len() {
        0 => RubyType::NilClass,
        1 => {
            let expr = results.first().unwrap();
            match expr {
                ast::OutgoingBindingExpression::As(e) => match e.ty {
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
                },
                ast::OutgoingBindingExpression::Utf8Str(_) => RubyType::String,
                _ => panic!("failed to decode results, unsupported type: {:?}", expr),
            }
        }
        _ => panic!("multiple return values are not supported"),
    }
}

fn translate_incoming(args: Array, param_types: &[RubyType]) -> Vec<wit::Value> {
    if args.length() != param_types.len() {
        panic!("incorrect arity");
    }
    args.into_iter()
        .zip(param_types)
        .map(|(arg, param_type)| match param_type {
            RubyType::Integer32 => {
                wit::Value::I32(arg.try_convert_to::<Integer>().unwrap().to_i32())
            }
            RubyType::Integer64 => {
                wit::Value::I64(arg.try_convert_to::<Integer>().unwrap().to_i64())
            }
            RubyType::Float32 => {
                wit::Value::F32(arg.try_convert_to::<Float>().unwrap().to_f64() as f32)
            }
            RubyType::Float64 => wit::Value::F64(arg.try_convert_to::<Float>().unwrap().to_f64()),
            RubyType::String => {
                wit::Value::String(arg.try_convert_to::<RString>().unwrap().to_string())
            }
            RubyType::Boolean | RubyType::NilClass | RubyType::Unsupported => {
                panic!("unsupported arg type")
            }
        })
        .collect()
}

fn translate_outgoing(native_results: Vec<wit::Value>) -> AnyObject {
    let results: Vec<AnyObject> = native_results
        .into_iter()
        .map(|r| match r {
            wit::Value::String(v) => RString::new_utf8(&v).into(),
            wit::Value::I32(v) => Integer::new(v.into()).into(),
            wit::Value::U32(v) => Integer::new(v.try_into().unwrap()).into(),
            wit::Value::I64(v) => Integer::new(v).into(),
            wit::Value::U64(v) => Integer::new(v.try_into().unwrap()).into(),
            wit::Value::F32(v) => Float::new(v.into()).into(),
            wit::Value::F64(v) => Float::new(v).into(),
        })
        .collect();

    match results.len() {
        0 => NilClass::new().into(),
        1 => results.first().unwrap().into(),
        _ => panic!("multiple return values are not supported"),
    }
}

wrappable_struct!(Function, FunctionWrapper, FUNCTION_WRAPPER);
class!(RubyFunction);

#[rustfmt::skip]
methods!(
    RubyFunction,
    itself,

    fn ruby_function_signature() -> Hash {
        let function = itself.get_data(&*FUNCTION_WRAPPER);

        let mut params = Array::new();
        for param in function.param_types.iter() {
            params.push(RString::new_utf8(&format!("{:?}", param)));
        }

        let result: AnyObject = function.result_type.into();

        let mut signature = Hash::new();
        signature.store(Symbol::new("params"), params);
        signature.store(Symbol::new("result"), result);

        signature
    }
);

pub extern "C" fn ruby_function_call(
    argc: r::types::Argc,
    argv: *const AnyObject,
    mut itself: AnyObject,
) -> AnyObject {
    // TODO: Remove this section when rutie `methods!` macro has support for variadic functions
    // https://github.com/danielpclark/rutie/blob/1c951b59e00944d305ca425267c54115c8c1bb86/README.md#variadic-functions--splat-operator
    let args_raw = r::types::Value::from(0);
    unsafe {
        let p_argv: *const r::types::Value = mem::transmute(argv);
        rubysys::class::rb_scan_args(
            argc,
            p_argv,
            r::util::str_to_cstring("*").as_ptr(),
            &args_raw,
        )
    };
    let args = Array::from(args_raw);
    // ---
    let function = itself.get_data_mut(&*FUNCTION_WRAPPER);

    let args_native = translate_incoming(args, &function.param_types);
    let results_native = function.call(&args_native[..]);
    translate_outgoing(results_native)
}

pub fn ruby_init() {
    Module::from_existing("Wasmtime").define(|module| {
        module
            .define_nested_class("Function", None)
            .define(|class| {
                class.def("signature", ruby_function_signature);
                class.def("call", ruby_function_call);
            });
    });
}

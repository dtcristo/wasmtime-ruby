use lazy_static::lazy_static;
use rutie as r;
use rutie::rubysys;
use rutie::{
    class, methods, wrappable_struct, AnyObject, Array, Hash, Module, Object, RString, Symbol,
};
use std::mem;
use std::rc::Rc;
use wasm_webidl_bindings::ast;
use wasmtime as w;
use wasmtime_interface_types as wit;

use crate::ruby_type::RubyType;
use crate::wasm_value::WasmValue;

pub struct Function {
    instance: w::HostRef<w::Instance>,
    module_data: Rc<wit::ModuleData>,
    export_name: String,
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

    pub fn call(&mut self, args: &[WasmValue]) -> Vec<WasmValue> {
        let args_native: Vec<wit::Value> = args.iter().map(|wv| wv.clone().into()).collect();
        self.module_data
            .invoke_export(&mut self.instance, &self.export_name, &args_native)
            .expect("unable to invoke export")
            .into_iter()
            .map(|v| v.into())
            .collect()
    }

    pub fn into_ruby(self) -> RubyFunction {
        Module::from_existing("Wasmtime")
            .get_nested_class("Function")
            .wrap_data(self, &*FUNCTION_WRAPPER)
    }
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

        let result: AnyObject = match function.result_types.len() {
            0 => RubyType::NilClass.into(),
            1 => function.result_types.iter().next().unwrap().clone().into(),
            _ => {
                let mut results = Array::new();
                for r in function.result_types.iter() {
                    let object: AnyObject = r.clone().into();
                    results.push(object);
                }
                results.into()
            },
        };

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
    let raw_args = r::types::Value::from(0);
    unsafe {
        let p_argv: *const r::types::Value = mem::transmute(argv);
        rubysys::class::rb_scan_args(
            argc,
            p_argv,
            r::util::str_to_cstring("*").as_ptr(),
            &raw_args,
        )
    };
    let args = Array::from(raw_args);
    // ---
    let function = itself.get_data_mut(&*FUNCTION_WRAPPER);
    let wasm_args: Vec<WasmValue> = args.into_iter().map(|o| o.into()).collect();

    let results = function.call(&wasm_args[..]);

    match results.len() {
        0 => r::NilClass::new().into(),
        1 => results.into_iter().next().unwrap().into(),
        _ => {
        let mut results_array = Array::new();
        for result in results.into_iter() {
            let object: AnyObject = result.into();
            results_array.push(object);
        }
        results_array.into()
    }
}
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

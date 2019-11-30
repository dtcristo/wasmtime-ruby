use lazy_static::lazy_static;
use rutie::{class, methods, wrappable_struct, AnyObject, Array, Module, Object, RString};
use std::fs;
use wasm_webidl_bindings::ast;
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

        // let mut handle = self.instance.borrow().handle().clone();
        // exports.iter().for_each(|export| {
        //     let export_binding = self
        //         .module_data
        //         .binding_for_export(&mut handle, export)
        //         .unwrap();
        //     let params = export_binding.param_bindings().unwrap();
        //     let results = export_binding.result_bindings().unwrap();
        //     dbg!(decode_params(&params));
        //     dbg!(decode_results(&results));
        // });

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

fn decode_params(params: &[ast::IncomingBindingExpression]) -> Vec<String> {
    params
        .iter()
        .map(|expr| match expr {
            ast::IncomingBindingExpression::As(e) => match e.ty {
                walrus::ValType::I32 => format!("Integer: {:?}", e.ty),
                walrus::ValType::I64 => format!("Integer: {:?}", e.ty),
                walrus::ValType::F32 => format!("Float: {:?}", e.ty),
                walrus::ValType::F64 => format!("Float: {:?}", e.ty),
                walrus::ValType::V128 | walrus::ValType::Anyref => {
                    format!("Unsupported: {:?}", e.ty)
                }
            },
            ast::IncomingBindingExpression::AllocUtf8Str(_) => format!("String"),
            _ => panic!("unsupported incoming binding expr {:?}", expr),
        })
        .collect()
}

fn decode_results(results: &[ast::OutgoingBindingExpression]) -> Vec<String> {
    results
        .iter()
        .map(|expr| match expr {
            ast::OutgoingBindingExpression::As(e) => match e.ty {
                ast::WebidlTypeRef::Scalar(ast::WebidlScalarType::UnsignedLong) => {
                    format!("Integer(U32): {:?}", e.ty)
                }
                ast::WebidlTypeRef::Scalar(ast::WebidlScalarType::Long) => {
                    format!("Integer(I32): {:?}", e.ty)
                }
                ast::WebidlTypeRef::Scalar(ast::WebidlScalarType::LongLong) => {
                    format!("Integer(I64): {:?}", e.ty)
                }
                ast::WebidlTypeRef::Scalar(ast::WebidlScalarType::UnsignedLongLong) => {
                    format!("Integer(U64): {:?}", e.ty)
                }
                ast::WebidlTypeRef::Scalar(ast::WebidlScalarType::Float) => {
                    format!("Float(F32): {:?}", e.ty)
                }
                ast::WebidlTypeRef::Scalar(ast::WebidlScalarType::Double) => {
                    format!("Float(F64): {:?}", e.ty)
                }
                _ => format!("Unsupported: {:?}", e.ty),
            },
            ast::OutgoingBindingExpression::Utf8Str(_) => format!("String"),
            _ => panic!("unsupported outgoing binding expr {:?}", expr),
        })
        .collect()
}

wrappable_struct!(Instance, InstanceWrapper, INSTANCE_WRAPPER);
class!(RubyInstance);

#[rustfmt::skip]
methods!(
    RubyInstance,
    itself,

    fn ruby_instance_new(path: RString) -> AnyObject {
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
                class.def("invoke", ruby_instance_invoke);
            });
    });
}

use lazy_static::lazy_static;
use rutie as r;
use rutie::rubysys;
use rutie::{
    class, methods, wrappable_struct, AnyObject, Array, Float, Hash, Integer, Module, NilClass,
    Object, RString, Symbol,
};
use std::mem;
use wasmtime as w;

use crate::vm::*;

pub struct Func {
    func: w::Func,
}

impl Func {
    pub fn new(func: w::Func) -> Self {
        Func { func }
    }

    pub fn call(&mut self, args: &[w::Val]) -> Vec<w::Val> {
        self.func.call(args).expect("failed to call func").to_vec()
    }

    pub fn into_ruby(self) -> RubyFunc {
        Module::from_existing("Wasmtime")
            .get_nested_class("Func")
            .wrap_data(self, &*FUNC_WRAPPER)
    }

    fn parse_param_types(&self) -> Vec<RubyType> {
        self.func
            .ty()
            .params()
            .iter()
            .map(|val_type| val_type_to_ruby_type(val_type))
            .collect()
    }

    fn parse_result_type(&self) -> RubyType {
        match self.func.ty().results().len() {
            0 => RubyType::NilClass,
            1 => val_type_to_ruby_type(self.func.ty().results().first().unwrap()),
            _ => raise("StandardError", "multiple return values are not supported"),
        }
    }
}

fn val_type_to_ruby_type(val_type: &w::ValType) -> RubyType {
    match val_type {
        w::ValType::I32 => RubyType::Integer32,
        w::ValType::I64 => RubyType::Integer64,
        w::ValType::F32 => RubyType::Float32,
        w::ValType::F64 => RubyType::Float64,
        _ => RubyType::Unsupported,
    }
}

#[derive(Debug, Copy, Clone)]
enum RubyType {
    Integer32,
    Integer64,
    Float32,
    Float64,
    // String,
    // Boolean,
    NilClass,
    Unsupported,
}

impl Into<AnyObject> for RubyType {
    fn into(self) -> AnyObject {
        RString::new_utf8(&format!("{:?}", self)).into()
    }
}

fn translate_incoming(args: Array, param_types: &[RubyType]) -> Vec<w::Val> {
    if args.length() != param_types.len() {
        raise(
            "ArgumentError",
            &format!(
                "wrong number of arguments (given {}, expected {})",
                args.length(),
                param_types.len()
            ),
        )
    }
    args.into_iter()
        .zip(param_types)
        .map(|(arg, param_type)| match param_type {
            RubyType::Integer32 => w::Val::I32(
                arg.try_convert_to::<Integer>()
                    .expect("failed to convert integer")
                    .to_i32(),
            ),
            RubyType::Integer64 => w::Val::I64(
                arg.try_convert_to::<Integer>()
                    .expect("failed to convert integer")
                    .to_i64(),
            ),
            RubyType::Float32 => w::Val::F32(
                (arg.try_convert_to::<Float>()
                    .expect("failed to convert float")
                    .to_f64() as f32).to_bits(),
            ),
            RubyType::Float64 => w::Val::F64(
                arg.try_convert_to::<Float>()
                    .expect("failed to convert float")
                    .to_f64().to_bits(),
            ),
            RubyType::NilClass | RubyType::Unsupported => {
                raise(
                    "StandardError",
                    &format!("unsupported arg type: {:?}", param_type),
                )
            }
        })
        .collect()
}

fn translate_outgoing(native_results: Vec<w::Val>) -> AnyObject {
    let results: Vec<AnyObject> = native_results
        .into_iter()
        .map(|r| match r {
            w::Val::I32(v) => Integer::new(v.into()).into(),
            w::Val::I64(v) => Integer::new(v).into(),
            w::Val::F32(v) => Float::new(f32::from_bits(v).into()).into(),
            w::Val::F64(v) => Float::new(f64::from_bits(v)).into(),
            _ => raise("StandardError", &format!("unsupported value: {:?}", r)),
        })
        .collect();

    match results.len() {
        0 => NilClass::new().into(),
        1 => results.first().unwrap().into(),
        _ => raise("StandardError", "multiple return values are not supported"),
    }
}

wrappable_struct!(Func, FuncWrapper, FUNC_WRAPPER);
class!(RubyFunc);

#[rustfmt::skip]
methods!(
    RubyFunc,
    itself,

    fn ruby_func_signature() -> Hash {
        let func = itself.get_data(&*FUNC_WRAPPER);

        let mut param_types = Array::new();
        for param_type in func.parse_param_types().iter() {
            param_types.push(RString::new_utf8(&format!("{:?}", param_type)));
        }

        let result_type: AnyObject = func.parse_result_type().into();

        let mut signature = Hash::new();
        signature.store(Symbol::new("params"), param_types);
        signature.store(Symbol::new("result"), result_type);

        signature
    }
);

pub extern "C" fn ruby_func_call(
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
    let func = itself.get_data_mut(&*FUNC_WRAPPER);

    let args_native = translate_incoming(args, &func.parse_param_types());
    let results_native = func.call(&args_native[..]);
    translate_outgoing(results_native)
}

pub fn ruby_init() {
    Module::from_existing("Wasmtime").define(|module| {
        module.define_nested_class("Func", None).define(|class| {
            class.def("signature", ruby_func_signature);
            class.def("call", ruby_func_call);
        });
    });
}

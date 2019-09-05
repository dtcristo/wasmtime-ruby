mod raw_instance;
mod wasm_value;

use lazy_static::lazy_static;
use rutie::{class, methods, module, wrappable_struct, AnyObject, Array, Module, Object, RString};

use crate::raw_instance::RawInstance;
use crate::wasm_value::WasmValue;

wrappable_struct!(RawInstance, InstanceWrapper, INSTANCE_WRAPPER);
module!(Wasmtime);
class!(Instance);

#[rustfmt::skip]
methods!(
    Instance,
    itself,

    fn instance_new(path: RString) -> AnyObject {
        let instance = RawInstance::new(path.unwrap().to_string());
        Module::from_existing("Wasmtime")
            .get_nested_class("Instance")
            .wrap_data(instance, &*INSTANCE_WRAPPER)
    }

    fn instance_exports() -> Array {
        let instance = itself.get_data_mut(&*INSTANCE_WRAPPER);
        let mut exports = Array::new();
        instance.exports().iter().for_each(|export| {
            exports.push(RString::new_utf8(export));
        });
        exports
    }

    fn instance_invoke(export: RString, args: Array) -> AnyObject {
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

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_native() {
    Module::from_existing("Wasmtime").define(|wasmtime| {
        wasmtime
            .define_nested_class("Instance", None)
            .define(|instance| {
                instance.def_self("new", instance_new);
                instance.def("exports", instance_exports);
                instance.def("invoke", instance_invoke);
            });
    });
}

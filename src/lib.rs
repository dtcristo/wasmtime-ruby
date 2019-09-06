mod instance;
mod wasm_value;

use lazy_static::lazy_static;
use rutie::{class, methods, module, wrappable_struct, AnyObject, Array, Module, Object, RString};

use crate::instance::Instance;
use crate::wasm_value::WasmValue;

wrappable_struct!(Instance, InstanceWrapper, INSTANCE_WRAPPER);
module!(RubyWasmtime);
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

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_native() {
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

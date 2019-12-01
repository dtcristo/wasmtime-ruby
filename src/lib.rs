mod function;
mod instance;
mod memory;
mod ruby_type;
mod wasm_value;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_native() {
    instance::ruby_init();
    function::ruby_init();
}

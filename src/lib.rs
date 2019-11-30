mod instance;
mod wasm_value;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_native() {
    instance::ruby_init();
}

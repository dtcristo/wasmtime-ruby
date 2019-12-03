mod function;
mod instance;
mod memory;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_native() {
    instance::ruby_init();
    function::ruby_init();
}

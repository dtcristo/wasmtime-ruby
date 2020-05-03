mod func;
mod instance;
mod memory;
mod vm;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_native() {
    std::panic::set_hook(Box::new(|panic_info| {
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            vm::raise("StandardError", s)
        } else {
            vm::raise("StandardError", &format!("{:?}", panic_info))
        }
    }));

    instance::ruby_init();
    func::ruby_init();
}

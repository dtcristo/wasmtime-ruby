use rutie::{Class, VM};

pub fn raise(exception: &str, message: &str) -> ! {
    VM::raise(Class::from_existing(exception), message);
    loop {}
}

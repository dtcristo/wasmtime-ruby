use helix::*;

ruby! {
    class Wasmtime {
        def hello() -> String {
            String::from("Hello from wasmtime!")
        }
    }
}

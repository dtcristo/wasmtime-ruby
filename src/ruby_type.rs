use rutie::{AnyObject, RString};

#[derive(Debug, Clone)]
pub enum RubyType {
    Integer32,
    Integer64,
    Float32,
    Float64,
    String,
    Boolean,
    NilClass,
    Unsupported,
}

impl Into<AnyObject> for RubyType {
    fn into(self) -> AnyObject {
        RString::new_utf8(&format!("{:?}", self)).into()
    }
}

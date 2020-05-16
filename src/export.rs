use rutie::{AnyObject, Object};

use crate::func::Func;
use crate::memory::Memory;

pub enum Export {
    Func(Func),
    Memory(Memory),
}

impl Into<AnyObject> for Export {
    fn into(self) -> AnyObject {
        match self {
            Export::Func(func) => func.into_ruby().value().into(),
            Export::Memory(memory) => memory.into_ruby().value().into(),
        }
    }
}

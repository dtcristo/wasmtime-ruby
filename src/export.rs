use rutie::{AnyObject, Object};

use crate::func::{Func, RubyFunc};
use crate::memory::{Memory, RubyMemory};

pub enum Export {
    Func(Func),
    Memory(Memory),
}

impl From<Export> for AnyObject {
    fn from(export: Export) -> Self {
        match export {
            Export::Func(func) => RubyFunc::from(func).value().into(),
            Export::Memory(memory) => RubyMemory::from(memory).value().into(),
        }
    }
}

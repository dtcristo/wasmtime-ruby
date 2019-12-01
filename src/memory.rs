pub struct Memory {
    pub export_name: String,
}

impl Memory {
    pub fn new(export_name: String) -> Self {
        Memory { export_name }
    }
}

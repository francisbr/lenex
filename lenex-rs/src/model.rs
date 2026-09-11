pub struct Lenex {
    // opaque for now — nested domain model is a follow-up task
}

impl Lenex {
    /// Build an empty document from scratch.
    pub fn new() -> Self {
        Lenex {}
    }
}

impl Default for Lenex {
    fn default() -> Self {
        Self::new()
    }
}

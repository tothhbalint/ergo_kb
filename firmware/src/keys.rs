pub struct KeyDescriptor {}

pub(crate) struct KeyMap {}

impl KeyMap {
    pub(crate) fn new() -> Self {
        KeyMap {}
    }

    pub(crate) fn get_key(&self, row: usize, col: usize) {}
}

pub struct KeyReport {
    keymap: KeyMap,
}

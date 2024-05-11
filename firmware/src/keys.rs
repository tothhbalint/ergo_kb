pub struct KeyDescriptor {

}

impl KeyDescriptor {
    pub fn new() -> Self {
        KeyDescriptor{}
    }

}

pub(crate) struct KeyMap {}

impl KeyMap {
    pub(crate) fn new() -> Self {
        KeyMap {}
    }

    pub(crate) fn get_key(&self, row: usize, col: usize) ->KeyReport {
        let key_descriptor = KeyDescriptor::new();
        return key_descriptor;
    }
}

pub struct KeyReport {
    keymap: KeyMap,
}

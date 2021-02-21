use gdnative::prelude::*;

// Utilized https://godot-rust.github.io/book/introduction.html heavily

#[derive(NativeClass)]
#[inherit(Node)]
pub struct GoreKill;

#[methods]
impl GoreKill {
    fn new(_owner: &Node) -> Self {
        Self
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("HELLO GORE_KILL");
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<GoreKill>();
}

godot_init!(init);

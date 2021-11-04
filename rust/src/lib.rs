use gdnative::prelude::{godot_init, InitHandle};
mod bush;
mod game;
mod player;
mod world;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<bush::Bush>();
    handle.add_class::<game::Game>();
    handle.add_class::<player::Player>();
    handle.add_class::<world::World>();
}

// macros that create the entry-points of the dynamic library.
godot_init!(init);

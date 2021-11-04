use gdnative::prelude::*;
use gdnative_api::*;

/// The Bush "class"
#[derive(NativeClass)]
#[inherit(StaticBody2D)]
#[register_with(Self::register_builder)]
pub struct Bush {
    name: String,
}

// __One__ `impl` block can have the `#[methods]` attribute, which will generate
// code to automatically bind any exported methods to Godot.
#[methods]
impl Bush {
    // Register the builder for methods, properties and/or signals.
    fn register_builder(_builder: &ClassBuilder<Self>) {
        godot_print!("Bush builder is registered!");
    }

    /// The "constructor" of the class.
    fn new(_owner: &StaticBody2D) -> Self {
        godot_print!("Bush is created!");
        Bush {
            name: "".to_string(),
        }
    }

    // In order to make a method known to Godot, the #[export] attribute has to be used.
    // In Godot script-classes do not actually inherit the parent class.
    // Instead they are "attached" to the parent object, called the "owner".
    // The owner is passed to every single exposed method.
    #[export]
    fn _ready(&mut self, _owner: &StaticBody2D) {
        // The `godot_print!` macro works like `println!` but prints to the Godot-editor
        // output tab as well.
        self.name = "Bush".to_string();
        godot_print!("{} is ready!", self.name);
    }

    // This function will be called in every frame
    #[export]
    fn _process(&self, _owner: &StaticBody2D, _delta: f64) {
        // godot_print!("Inside {} _process(), delta is {}", self.name, delta);
    }
}

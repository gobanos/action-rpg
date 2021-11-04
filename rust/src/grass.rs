use gdnative::prelude::*;
use gdnative_api::*;

/// The Grass "class"
#[derive(NativeClass)]
#[inherit(Node2D)]
#[register_with(Self::register_builder)]
pub struct Grass {
    name: String,
}

// __One__ `impl` block can have the `#[methods]` attribute, which will generate
// code to automatically bind any exported methods to Godot.
#[methods]
impl Grass {
    // Register the builder for methods, properties and/or signals.
    fn register_builder(_builder: &ClassBuilder<Self>) {
        godot_print!("Grass builder is registered!");
    }

    /// The "constructor" of the class.
    fn new(_owner: &Node2D) -> Self {
        godot_print!("Grass is created!");
        Grass {
            name: "".to_string(),
        }
    }

    // In order to make a method known to Godot, the #[export] attribute has to be used.
    // In Godot script-classes do not actually inherit the parent class.
    // Instead they are "attached" to the parent object, called the "owner".
    // The owner is passed to every single exposed method.
    #[export]
    fn _ready(&mut self, _owner: &Node2D) {
        // The `godot_print!` macro works like `println!` but prints to the Godot-editor
        // output tab as well.
        self.name = "Grass".to_string();
        godot_print!("{} is ready!", self.name);
    }

    // This function will be called in every frame
    #[export]
    fn _process(&self, owner: &Node2D, _delta: f64) {
        let input = Input::godot_singleton();

        if input.is_action_just_pressed("attack") {
            let resource_loader = ResourceLoader::godot_singleton();

            unsafe {
                let grass_effect_scene = resource_loader
                    .load("res://scenes/GrassEffect.tscn", "", false)
                    .unwrap()
                    .try_cast::<PackedScene>()
                    .unwrap();

                let grass_effect = grass_effect_scene.assume_safe().instance(0).unwrap();

                let world = owner
                    .get_tree()
                    .unwrap()
                    .assume_safe()
                    .current_scene()
                    .unwrap();

                world.assume_safe().add_child(grass_effect, false);

                grass_effect
                    .assume_safe()
                    .cast::<Node2D>()
                    .unwrap()
                    .set_global_position(owner.global_position());
            }

            owner.queue_free();
        }
    }
}

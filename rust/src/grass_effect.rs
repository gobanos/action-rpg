use gdnative::prelude::*;
use gdnative_api::utils::NodeExt;
use gdnative_api::*;

/// The GrassEffect "class"
#[derive(NativeClass)]
#[inherit(Node2D)]
#[register_with(Self::register_builder)]
pub struct GrassEffect {
    animated_sprite: Option<Ref<AnimatedSprite>>,
}

// __One__ `impl` block can have the `#[methods]` attribute, which will generate
// code to automatically bind any exported methods to Godot.
#[methods]
impl GrassEffect {
    // Register the builder for methods, properties and/or signals.
    fn register_builder(_builder: &ClassBuilder<Self>) {
        godot_print!("GrassEffect builder is registered!");
    }

    /// The "constructor" of the class.
    fn new(_owner: &Node2D) -> Self {
        GrassEffect {
            animated_sprite: None,
        }
    }

    /// Access the AnimatedSprite node.
    /// Panics if called before `_ready`.
    fn animated_sprite(&self) -> TRef<AnimatedSprite> {
        unsafe { self.animated_sprite.as_ref().unwrap().assume_safe() }
    }

    // In order to make a method known to Godot, the #[export] attribute has to be used.
    // In Godot script-classes do not actually inherit the parent class.
    // Instead they are "attached" to the parent object, called the "owner".
    // The owner is passed to every single exposed method.
    #[export]
    fn _ready(&mut self, owner: &Node2D) {
        self.animated_sprite = Some(unsafe {
            owner
                .get_node_as::<AnimatedSprite>("AnimatedSprite")
                .expect("AnimatedSprite node not found")
                .claim()
        });

        self.animated_sprite().play("Animate", false);
    }

    // This function will be called in every frame
    #[export]
    fn _process(&self, _owner: &Node2D, _delta: f64) {
        // godot_print!("Inside {} _process(), delta is {}", self.name, delta);
    }

    // This function will be called in every frame
    #[export]
    fn on_animated_sprite_animation_finished(&self, owner: &Node2D) {
        owner.queue_free();
    }
}

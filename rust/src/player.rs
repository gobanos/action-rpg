use gdnative::prelude::*;
use gdnative_api::*;

const ACCELERATION: f32 = 500.0;
const FRICTION: f32 = 500.0;
const MAX_SPEED: f32 = 80.0;

/// The Player "class"
#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
#[register_with(Self::register_builder)]
pub struct Player {
    velocity: Vector2,
}

// __One__ `impl` block can have the `#[methods]` attribute, which will generate
// code to automatically bind any exported methods to Godot.
#[methods]
impl Player {
    // Register the builder for methods, properties and/or signals.
    fn register_builder(_builder: &ClassBuilder<Self>) {
        godot_print!("Player builder is registered!");
    }

    /// The "constructor" of the class.
    fn new(_owner: &KinematicBody2D) -> Self {
        godot_print!("Player is created!");
        Player {
            velocity: Vector2::zero(),
        }
    }

    // This function will be called in every frame
    #[export]
    unsafe fn _physics_process(&mut self, owner: &KinematicBody2D, delta: f32) {
        let input = Input::godot_singleton();
        let input_vector = Vector2::new(
            (input.get_action_strength("ui_right") - input.get_action_strength("ui_left")) as f32,
            (input.get_action_strength("ui_down") - input.get_action_strength("ui_up")) as f32,
        );

        self.velocity = if input_vector != Vector2::zero() {
            self.velocity
                .move_towards(input_vector.normalize() * MAX_SPEED, ACCELERATION * delta)
        } else {
            self.velocity
                .move_towards(Vector2::zero(), FRICTION * delta)
        };

        self.velocity = owner.move_and_slide(
            self.velocity,
            Vector2::zero(),
            false,
            4,
            std::f64::consts::FRAC_PI_4,
            true,
        );
    }
}

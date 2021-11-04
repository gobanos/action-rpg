use gdnative::prelude::*;
use gdnative_api::utils::NodeExt;
use gdnative_api::*;
use strum::AsRefStr;

const ACCELERATION: f32 = 500.0;
const FRICTION: f32 = 500.0;
const MAX_SPEED: f32 = 80.0;

/// The Player "class"
#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
#[register_with(Self::register_builder)]
pub struct Player {
    velocity: Vector2,
    animation_player: Option<Ref<AnimationPlayer>>,
    animation: PlayerAnimation,
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
            animation_player: None,
            animation: PlayerAnimation::default(),
        }
    }

    /// Access the AnimationPlayer node.
    /// Panics if called before `_ready`.
    fn animation_player(&self) -> TRef<AnimationPlayer> {
        unsafe { self.animation_player.unwrap().assume_safe() }
    }

    #[export]
    fn _ready(&mut self, owner: &KinematicBody2D) {
        self.animation_player = Some(unsafe {
            owner
                .get_node_as::<AnimationPlayer>("AnimationPlayer")
                .expect("AnimationPlayer node not found")
                .claim()
        });

        self.animation_player()
            .play(self.animation.name(), -1.0, 1.0, false);
    }

    // This function will be called in every frame
    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody2D, delta: f32) {
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

        self.animation.update_from_velocity(self.velocity);
        self.animation_player()
            .play(self.animation.name(), -1.0, 1.0, false);

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

#[derive(Debug, Copy, Clone)]
struct PlayerAnimation {
    status: Status,
    direction: Direction,
}

impl PlayerAnimation {
    fn name(self) -> String {
        format!("{}{}", self.status.as_ref(), self.direction.as_ref())
    }

    fn update_from_velocity(&mut self, velocity: Vector2) {
        *self = if velocity == Vector2::zero() {
            PlayerAnimation {
                status: Status::Idle,
                direction: self.direction,
            }
        } else {
            let Vector2 { x, y, .. } = velocity;
            if x.abs() >= y.abs() {
                PlayerAnimation {
                    status: Status::Run,
                    direction: if x > 0.0 {
                        Direction::Right
                    } else {
                        Direction::Left
                    },
                }
            } else {
                PlayerAnimation {
                    status: Status::Run,
                    direction: if y > 0.0 {
                        Direction::Down
                    } else {
                        Direction::Up
                    },
                }
            }
        };
    }
}

impl Default for PlayerAnimation {
    fn default() -> Self {
        PlayerAnimation {
            status: Status::Idle,
            direction: Direction::Right,
        }
    }
}

#[derive(AsRefStr, Debug, Copy, Clone)]
enum Status {
    Idle,
    Run,
}

#[derive(AsRefStr, Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

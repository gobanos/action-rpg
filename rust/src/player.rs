use gdnative::prelude::*;
use gdnative_api::utils::NodeExt;
use gdnative_api::*;

const ACCELERATION: f32 = 500.0;
const FRICTION: f32 = 500.0;
const MAX_SPEED: f32 = 80.0;

/// The Player "class"
#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
#[register_with(Self::register_builder)]
pub struct Player {
    state: State,
    velocity: Vector2,
    animation_tree: Option<Ref<AnimationTree>>,
    animation_state: Option<Ref<AnimationNodeStateMachinePlayback>>,
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
            state: State::Move,
            velocity: Vector2::zero(),
            animation_tree: None,
            animation_state: None,
        }
    }

    /// Access the AnimationTree node.
    /// Panics if called before `_ready`.
    fn animation_tree(&self) -> TRef<AnimationTree> {
        unsafe { self.animation_tree.as_ref().unwrap().assume_safe() }
    }

    /// Access the AnimationNodeStateMachinePlayback node.
    /// Panics if called before `_ready`.
    fn animation_state(&self) -> TRef<AnimationNodeStateMachinePlayback> {
        unsafe { self.animation_state.as_ref().unwrap().assume_safe() }
    }

    #[export]
    fn _ready(&mut self, owner: &KinematicBody2D) {
        self.animation_tree = Some(unsafe {
            owner
                .get_node_as::<AnimationTree>("AnimationTree")
                .expect("AnimationTree node not found")
                .claim()
        });
        self.animation_tree().set_active(true);
        self.animation_state = Some(
            self.animation_tree()
                .get("parameters/playback")
                .try_to_object::<AnimationNodeStateMachinePlayback>()
                .expect(
                    "AnimationTree parameters/playback is not a AnimationNodeStateMachinePlayback",
                ),
        );
    }

    // This function will be called in every frame
    #[export]
    fn _process(&mut self, owner: &KinematicBody2D, delta: f32) {
        match self.state {
            State::Move => self.process_move(owner, delta),
            State::Attack => self.process_attack(owner, delta),
            State::Roll => self.process_roll(owner, delta),
        }
    }

    fn process_move(&mut self, owner: &KinematicBody2D, delta: f32) {
        let input = Input::godot_singleton();
        let input_vector = Vector2::new(
            (input.get_action_strength("ui_right") - input.get_action_strength("ui_left")) as f32,
            (input.get_action_strength("ui_down") - input.get_action_strength("ui_up")) as f32,
        );

        self.velocity = if input_vector != Vector2::zero() {
            self.animation_tree()
                .set("parameters/Idle/blend_position", input_vector);
            self.animation_tree()
                .set("parameters/Run/blend_position", input_vector);
            self.animation_tree()
                .set("parameters/Attack/blend_position", input_vector);
            self.animation_state().travel("Run");
            self.velocity
                .move_towards(input_vector.normalize() * MAX_SPEED, ACCELERATION * delta)
        } else {
            self.animation_state().travel("Idle");
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

        if input.is_action_just_pressed("attack") {
            self.state = State::Attack;
        }
    }

    fn process_attack(&mut self, _owner: &KinematicBody2D, _delta: f32) {
        self.animation_state().travel("Attack");
    }

    #[export]
    fn attack_animation_finished(&mut self, _owner: &KinematicBody2D) {
        self.state = State::Move;
        self.velocity = Vector2::zero();
    }

    fn process_roll(&mut self, _owner: &KinematicBody2D, _delta: f32) {}
}

#[derive(Debug, Copy, Clone)]
enum State {
    Move,
    Roll,
    Attack,
}

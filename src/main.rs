use bevy::{prelude::*, render::pass::ClearColor};

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "TOMI".to_string(),
            ..Default::default()
        })
        .add_default_plugins()
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup.system())
        .add_system(player_movement.system())
        .add_system(movement.system())
        .run();
}

// Components

/// Component for any movable
#[derive(Copy, Clone)]
struct Movable {
    speed: f32,
    movement: MovementState,
}

impl Movable {
    pub fn with_speed(speed: f32) -> Movable {
        Movable {
            speed,
            movement: MovementState::Stationary
        }
    }
}

/// The current state of the any movable
#[derive(Copy, Clone)]
enum MovementState {
    Stationary,
    Walking(Direction),
}

/// An enum representing a direction within the game
#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Marker Component with no state, only used to mark
/// which movable to update from user input
#[derive(Copy, Clone)]
struct PlayerMovable {}

// Systems

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("assets/tomi.png").unwrap();
    commands
        // Cameras
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        // Tomi
        .spawn(SpriteComponents {
            material: materials.add(texture_handle.into()),
            ..Default::default()
        })
        .with(Movable::with_speed(300.0))
        .with(PlayerMovable {});
}

/// Updates Movable based on keyboard input
fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&PlayerMovable, &mut Movable)>,
) {
    for (_, mut movement) in &mut query.iter() {
        movement.movement = if keyboard_input.pressed(KeyCode::Left) {
            MovementState::Walking(Direction::Left)
        } else if keyboard_input.pressed(KeyCode::Right) {
            MovementState::Walking(Direction::Right)
        } else if keyboard_input.pressed(KeyCode::Down) {
            MovementState::Walking(Direction::Down)
        } else if keyboard_input.pressed(KeyCode::Up) {
            MovementState::Walking(Direction::Up)
        } else {
            MovementState::Stationary
        };
    }
}

/// Updates on screen position based on the Movable
fn movement(time: Res<Time>, mut query: Query<(&Movable, &mut Transform)>) {
    for (movable, mut transform) in &mut query.iter() {
        if let MovementState::Walking(direction) = movable.movement {
            let translation = transform.translation_mut();
            let distance = time.delta_seconds * movable.speed;
            match direction {
                Direction::Left => *translation.x_mut() -= distance,
                Direction::Right => *translation.x_mut() += distance,
                Direction::Up => *translation.y_mut() += distance,
                Direction::Down => *translation.y_mut() -= distance,
            }
        }
    }
}

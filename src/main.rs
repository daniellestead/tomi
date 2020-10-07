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
        .add_system(ai_movement.system())
        .add_system(movement.system())
        .add_system(process_movable_tick.system())
        .run();
}

// Components

/// Component for any movable
#[derive(Copy, Clone)]
struct Movable {
    speed: f32,
    movement: MovementState,
    //    frame: usize
}

impl Movable {
    pub fn with_speed(speed: f32) -> Movable {
        Movable {
            speed,
            movement: MovementState::Stationary,
            //            frame: 0
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

/// Marker Component with no state, only used to mark
/// which movable to update from user input
#[derive(Copy, Clone)]
struct AiMovable {}

// Systems

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let ai_texture_handle = asset_server.load("assets/tomi_stationary.png").unwrap();
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/tomi_walking.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 4, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        // Cameras
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        // Tomi
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(1.0),
            ..Default::default()
        })
        .with(Movable::with_speed(300.0))
        .with(PlayerMovable {})
        .with(Timer::from_seconds(0.3, true))
        // Robot Tomi
        .spawn(SpriteComponents {
            material: materials.add(ai_texture_handle.into()),
            transform: Transform::from_translation(Vec3::new(150.0, -100.0, 0.0)),
            ..Default::default()
        })
        .with(Movable::with_speed(200.0))
        .with(AiMovable {})
        .with(Timer::from_seconds(2.0, true));
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

/// Updates an AI movable
fn ai_movement(mut query: Query<(&AiMovable, &Timer, &mut Movable)>) {
    for (_, timer, mut movement) in &mut query.iter() {
        if timer.finished {
            movement.movement = if let MovementState::Walking(Direction::Left) = movement.movement {
                MovementState::Walking(Direction::Right)
            } else {
                MovementState::Walking(Direction::Left)
            }
        }
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

/// Processes a timer tick for a movable, ie updates sprites for animations etc
fn process_movable_tick(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &Timer,
        &Movable,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (timer, movable, mut sprite, texture_atlas_handle) in &mut query.iter() {
        if timer.finished {
            if let MovementState::Walking(_) = movable.movement {
                // Audio doesn't seem to work properly yet :(
                //audio_output.play(sounds.step);
                let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
                sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
            }
        }
    }
}

use bevy::{prelude::*, render::pass::ClearColor};

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "TOMI".to_string(),
            ..Default::default()
        })
        .add_default_plugins()
        // .add_resource(Scoreboard { score: 0 })
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup.system())
        // .add_system(animate_system.system())
        // .add_system(scoreboard_system.system())
        //.add_system(movement_system.system())
        .add_system(player_movement.system())
        .run();
}

// struct Scoreboard {
//     score: usize,
// }

//

//
//
// .with(Tomi { speed: 500.0, movement: MovementState::Stationary });
// enum MovementState {
//   Stationary,
//   Walking(direction: Direction),
//   Dead
// }
// enum Direction {
//   Up,
//   Down,
//   Left,
//   Right
// }
//
// .with(Tomi { speed: 500.0, movement: MovementState::Stationary });
//

// #[derive(Default)]
// struct Movement {
//     speed: f32,
// }
struct Movement {
    speed: f32,
    movement: MovementState,
}
enum MovementState {
    Stationary,
    Walking(Direction),
}
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Default for Movement {
    fn default() -> Self {
        Movement::Stationary
    }
}

// fn animate_system(
//     texture_atlases: Res<Assets<TextureAtlas>>,
//     mut scoreboard: ResMut<Scoreboard>,
//     mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
// ) {
//     for (timer, mut sprite, texture_atlas_handle) in &mut query.iter() {
//         if timer.finished {
//             let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
//             sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
//             scoreboard.score += 1;
//         }
//     }
// }

// fn scoreboard_system(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
//     for mut text in &mut query.iter() {
//         text.value = format!("Score: {}", scoreboard.score);
//     }
// }

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut textures: ResMut<Assets<Texture>>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // let texture_handle = asset_server
    //     .load_sync(
    //         &mut textures,
    //         "assets/blow_kiss.png",
    //     )
    //     .unwrap();
    let texture_handle = asset_server.load("assets/tomi.png").unwrap();
    // let texture = textures.get(&texture_handle).unwrap();
    // let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 3, 1);
    // let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        // Tomi
        .spawn(SpriteComponents {
            material: materials.add(texture_handle.into()),
            ..Default::default()
        })
        // .spawn(SpriteSheetComponents {
        //     texture_atlas: texture_atlas_handle,
        //     transform: Transform::from_scale(1.0),
        //     ..Default::default()
        // })
        .with(Movement::default());
    // .with(Timer::from_seconds(0.5, true))
    // Scoreboard
    // .spawn(TextComponents {
    //     text: Text {
    //         font: asset_server.load("assets/fonts/FiraSans-Bold.ttf").unwrap(),
    //         value: "Score:".to_string(),
    //         style: TextStyle {
    //             color: Color::rgb_u8(254, 209, 250),
    //             font_size: 40.0,
    //         },
    //     },
    //     style: Style {
    //         position_type: PositionType::Absolute,
    //         position: Rect {
    //             top: Val::Px(5.0),
    //             left: Val::Px(5.0),
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // });
}

fn player_movement(
    // Reads in from the keyboard and updates the MovementState
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Movement, &mut Transform)>,
) {
    for (movement, mut transform) in &mut query.iter() {
        let mut xdirection = 0.0;
        let mut ydirection = 0.0;
        if keyboard_input.pressed(KeyCode::Left) {
            xdirection -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            xdirection += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            ydirection -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            ydirection += 1.0;
        }

        let translation = transform.translation_mut();
        // Move horizontally
        *translation.x_mut() += time.delta_seconds * xdirection * movement.speed;
        // Move vertically
        *translation.y_mut() += time.delta_seconds * ydirection * movement.speed;
    }
}

// fn movement_system(
//     // Updates the position based on the current MovementState
// ) {
//
// }

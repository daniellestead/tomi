use bevy::{
    prelude::*,
    render::pass::ClearColor
};

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
                title: "TOMI".to_string(),
                ..Default::default()
            })
        .add_default_plugins()
        .add_resource(Scoreboard { score: 0 })
        .add_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .add_startup_system(setup.system())
        .add_system(animate_system.system())
        .add_system(scoreboard_system.system())
        .run();
}

struct Scoreboard {
    score: usize,
}

fn animate_system(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut scoreboard: ResMut<Scoreboard>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (timer, mut sprite, texture_atlas_handle) in &mut query.iter() {
        if timer.finished {
            let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
            scoreboard.score += 1;
        }
    }
}

fn scoreboard_system(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    for mut text in &mut query.iter() {
        text.value = format!("Score: {}", scoreboard.score);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server
        .load_sync(
            &mut textures,
            "assets/blow_kiss.png",
        )
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 3, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(1.0),
            ..Default::default()
        })
        .with(Timer::from_seconds(0.5, true))
        // Scoreboard
        .spawn(TextComponents {
            text: Text {
                font: asset_server.load("assets/fonts/FiraSans-Bold.ttf").unwrap(),
                value: "Score:".to_string(),
                style: TextStyle {
                    color: Color::rgb(0.2, 0.2, 0.8),
                    font_size: 40.0,
                },
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        });
}

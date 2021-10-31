use bevy::{core::FixedTimestep, prelude::*};

const FERRIS: &str = "ferris.png";
const ENV: &str = "tiles.png";
const TIME_STEP: f32 = 1. / 60.;

pub struct Materials {
    ferris: Handle<ColorMaterial>,
    env: Handle<ColorMaterial>,
}

struct Ferris;

struct Speed(f32);

impl Default for Speed {
    fn default() -> Self {
        Self(500.)
    }
}

fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Speed, &mut Transform), With<Ferris>>,
) {
    if let Ok((speed, mut transform)) = query.single_mut() {
        let x_dir = if keyboard_input.pressed(KeyCode::Left) {
            -1.
        } else if keyboard_input.pressed(KeyCode::Right) {
            1.
        } else {
            0.
        };

        let y_dir = if keyboard_input.pressed(KeyCode::Down) {
            -1.
        } else if keyboard_input.pressed(KeyCode::Up) {
            1.
        } else {
            0.
        };

        transform.translation.x += x_dir * speed.0 * TIME_STEP;
        transform.translation.y += y_dir * speed.0 * TIME_STEP;
    }
}

pub struct FerrisPlugin;

impl Plugin for FerrisPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(FerrisState::default())
            .add_system(movement.system())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.5))
                    .with_system(ferris_spawn.system()),
            );
    }
}

struct FerrisState {
    on: bool,
}

impl FerrisState {
    fn spawned(&mut self) {
        self.on = true;
    }
}

impl Default for FerrisState {
    fn default() -> Self {
        Self { on: false }
    }
}

fn ferris_spawn(
    mut commands: Commands,
    materials: Res<Materials>,
    mut ferris_state: ResMut<FerrisState>,
) {
    if !ferris_state.on {
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.ferris.clone(),
                ..Default::default()
            })
            .insert(Ferris)
            .insert(Speed::default());
        ferris_state.spawned();
    }
}

// #[derive(Component)]
struct Floor {
    speed: f32,
}

// #[derive(Component)]
enum Obstacle {
    Bottom(u8),
    Top(u8),
    Both(u8, u8),
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_xyz(0.0, -215.0, 0.0),
            sprite: Sprite::new(Vec2::new(120., 30.0)),
            ..Default::default()
        })
        .insert(Floor { speed: 500. });

    commands.insert_resource(Materials {
        ferris: materials.add(asset_server.load(FERRIS).into()),
        env: materials.add(asset_server.load(ENV).into()),
    });
}

fn main() {
    let mut app = App::build();
    app.add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "2D Bevs".into(),
            ..Default::default()
        })
        .add_plugin(FerrisPlugin)
        .add_startup_system(setup.system())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(env_movement_system.system()),
        );

    app.run();
}

fn env_movement_system(mut query: Query<(&Floor, &mut Transform)>) {
    // for (env, mut transform) in query.iter_mut() {
    //     let translation = &mut transform.translation;
    //     translation.x += env.speed * TIME_STEP;
    // }

    if let Ok((env, mut transform)) = query.single_mut() {
        transform.translation.x -= env.speed * TIME_STEP;
    }
}

use bevy::{core::FixedTimestep, prelude::*};

const FERRIS: &str = "ferris.png";
const TIME_STEP: f32 = 1. / 60.;

pub struct Materials {
    ferris: Handle<ColorMaterial>,
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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.insert_resource(Materials {
        ferris: materials.add(asset_server.load(FERRIS).into()),
    });
}

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "2D Bevs".into(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FerrisPlugin)
        .add_startup_system(setup.system())
        .run();
}

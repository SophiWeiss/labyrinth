use crate::motion::{Acceleration, MotionBundle};
use bevy::prelude::*;
use bevy_third_person_camera::ThirdPersonCameraTarget;

const MASS: f32 = 1.0;

pub struct BallPlugin;

#[derive(Component)]
pub struct Ball;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn)
            .add_systems(Update, keyboard_motion);
    }
}

fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::default()),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("#ff5471").unwrap(),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        ThirdPersonCameraTarget,
        MotionBundle::default(),
        Ball,
    ));
}

fn keyboard_motion(
    mut query: Query<&mut Acceleration, With<Ball>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    use KeyCode::{KeyA, KeyD, KeyS, KeyW};

    let acceleration = &mut query.single_mut();

    if input.all_pressed([KeyW, KeyD]) {
        acceleration.z = -1.0 / (MASS * 2.0f32.sqrt());
        acceleration.x = 1.0 / (MASS * 2.0f32.sqrt());
    } else if input.all_pressed([KeyW, KeyA]) {
        acceleration.z = -1.0 / (MASS * 2.0f32.sqrt());
        acceleration.x = -1.0 / (MASS * 2.0f32.sqrt());
    } else if input.all_pressed([KeyS, KeyD]) {
        acceleration.z = 1.0 / (MASS * 2.0f32.sqrt());
        acceleration.x = 1.0 / (MASS * 2.0f32.sqrt());
    } else if input.all_pressed([KeyS, KeyA]) {
        acceleration.z = 1.0 / (MASS * 2.0f32.sqrt());
        acceleration.x = -1.0 / (MASS * 2.0f32.sqrt());
    } else if input.pressed(KeyW) {
        acceleration.z = -1.0 / MASS;
    } else if input.pressed(KeyS) {
        acceleration.z = 1.0 / MASS;
    } else if input.pressed(KeyD) {
        acceleration.x = 1.0 / MASS;
    } else if input.pressed(KeyA) {
        acceleration.x = -1.0 / MASS;
    }

    if input.any_just_released([KeyW, KeyS]) {
        acceleration.z = 0.0;
    } else if input.any_just_released([KeyD, KeyA]) {
        acceleration.x = 0.0;
    }
}

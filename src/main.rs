use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

#[derive(Event, Default)]
struct CollisionEvent;

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

const BACKGROUND_COLOR: Color = Color::WHITE;
const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec3 = Vec3::new(100.0, 100.0, 0.0);
const BALL_SPEED: f32 = 300.0;
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.0, 0.0);

const G_ACC: f32 = -98.1;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(BALL_COLOR)),
            transform: Transform::from_translation(BALL_STARTING_POSITION).with_scale(BALL_SIZE),
            ..default()
        },
        Ball,
        Velocity(INITIAL_BALL_DIRECTION.normalize_or_zero() * BALL_SPEED),
    ));
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<FixedTime>) {
    for (mut transform, velocity) in &mut query {
        // println!("{}", &velocity.0.y);
        transform.translation.x += velocity.x * time_step.period.as_secs_f32();
        transform.translation.y += velocity.y * time_step.period.as_secs_f32();
    }
}

fn apply_gravity(mut query: Query<&mut Velocity, With<Ball>>, time_step: Res<FixedTime>) {
    let mut ball_velocity = query.single_mut();
    // println!("ball velocity {}", &ball_velocity.0.y);

    ball_velocity.0.y = ball_velocity.0.y + G_ACC * time_step.period.as_secs_f32();
}

fn apply_boundaries(query: Query<&Window>) {
    let window = query.single();

    println!(
        "window width {}, height {}",
        &window.resolution.width().to_string(),
        &window.resolution.height().to_string()
    )
}

fn move_ball(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Ball>>,
    time_step: Res<FixedTime>,
) {
    let mut ball_transform = query.single_mut();
    let mut x_direction: f32 = 0.0;

    if keyboard_input.pressed(KeyCode::A) {
        x_direction = -1.0;
    }

    if keyboard_input.pressed(KeyCode::D) {
        x_direction = 1.0;
    }

    ball_transform.translation.x =
        ball_transform.translation.x + x_direction * BALL_SPEED * time_step.period.as_secs_f32();
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, (setup, apply_boundaries))
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(FixedUpdate, (move_ball, apply_gravity, apply_velocity))
        .run();
}

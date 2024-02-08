use bevy::sprite::collide_aabb::collide;
use bevy::{math::vec2, prelude::*, sprite::MaterialMesh2dBundle};

pub struct StartupPlugin;
impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_objects)
            .add_systems(Update, (control_player, control_ai, control_ball))
            .add_systems(Update, check_collision);
    }
}

#[derive(Component)]
struct AI;

#[derive(Component)]
struct Paddle {
    speed: f32,
}

#[derive(Component)]
struct Ball {
    speed_x: f32,
    speed_y: f32,
}

fn spawn_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let rect = shape::Quad::new(vec2(45.0, 15.0));

    commands.spawn((
        Paddle { speed: 100.0 },
        MaterialMesh2dBundle {
            mesh: meshes.add(rect.into()).into(),
            material: materials.add(Color::BLUE.into()).into(),
            transform: Transform::from_translation(Vec3::new(0.0, -200.0, 0.0)).into(),
            ..default()
        },
    ));

    commands.spawn((
        AI,
        Paddle { speed: 100.0 },
        MaterialMesh2dBundle {
            mesh: meshes.add(rect.into()).into(),
            material: materials.add(Color::BLUE.into()).into(),
            transform: Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)).into(),
            ..default()
        },
    ));

    let circle = shape::Circle::new(25.0);

    commands.spawn((
        Ball {
            speed_x: 40.0,
            speed_y: -40.0,
        },
        MaterialMesh2dBundle {
            mesh: meshes.add(circle.into()).into(),
            material: materials.add(Color::RED.into()).into(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)).into(),
            ..default()
        },
    ));
}

fn control_player(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Paddle), Without<AI>>,
) {
    for (mut transform, paddle) in &mut query {
        if keys.pressed(KeyCode::A) {
            transform.translation.x -= paddle.speed * time.delta().as_secs_f32();
        }
        if keys.pressed(KeyCode::D) {
            transform.translation.x += paddle.speed * time.delta().as_secs_f32();
        }
    }
}

fn control_ai(time: Res<Time>, mut query: Query<(&mut Transform, &Paddle), With<AI>>) {
    for (mut transform, paddle) in &mut query {
        transform.translation.x += paddle.speed * time.delta().as_secs_f32();
    }
}

fn control_ball(time: Res<Time>, mut query: Query<(&mut Transform, &Ball)>) {
    for (mut transform, ball) in &mut query {
        transform.translation.x += ball.speed_x * time.delta().as_secs_f32();
        transform.translation.y += ball.speed_y * time.delta().as_secs_f32();
    }
}

fn check_collision(
    mut ball_query: Query<(&Transform, &mut Ball)>,
    paddle_query: Query<&Transform, With<Paddle>>,
) {
    if let Some((ball_transform, mut ball)) = ball_query.iter_mut().next() {
        let ball_size = Vec2 { x: 25.0, y: 25.0 };
        for paddle_transform in &paddle_query {
            let paddle_size = Vec2 { x: 45.0, y: 15.0 };
            let c = collide(
                ball_transform.translation,
                ball_size,
                paddle_transform.translation,
                paddle_size,
            );
            if let Some(_) = c {
                ball.speed_y = ball.speed_y * -1.0;
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, StartupPlugin))
        .run();
}

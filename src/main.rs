use bevy::prelude::*;

const PLAYER_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);


fn main() {
    App::new()
    .add_startup_system(setup_camera)
    .add_startup_system(spawn_player)
    .add_system(apply_speed.label("position_calculation"))
    .add_system(applied_gravity.before("position_calculation"))
    .add_system(handle_player_movement.before("position_calculation").label("handle_movement"))
    .add_system(inertie.after("handle_movement").before("position_calculation"))
    .add_plugins(DefaultPlugins).run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: PLAYER_COLOR,
                ..Default::default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Speed{x:0.0, y:0.0})
        .insert(Gravity)
        .insert(Player{jump_power : 40.0});
}

#[derive(Component, Debug)]
struct Speed{
    pub x : f32,
    pub y : f32
}

#[derive(Component)]
struct Gravity;

fn applied_gravity(mut query : Query<&mut Speed, With<Gravity>>){
    const GRAVITY: f32 = 0.9;
    const MAX_SPEED: f32 = -20.0;

    for mut speed in query.iter_mut(){
        if speed.y > MAX_SPEED{
            speed.y -= GRAVITY;
            if speed.y < MAX_SPEED{
                speed.y = MAX_SPEED;
            }
        }
    }
}

fn apply_speed(mut query: Query<(&mut Transform, &Speed)>){
    for (mut transform, speed) in query.iter_mut(){
        
        transform.translation.x += speed.x;
        transform.translation.y += speed.y;

        if transform.translation.y < -50.0{
            transform.translation.y = -50.0;
        }
    }
}

#[derive(Component)]
struct Player{
    jump_power : f32,
}

fn handle_player_movement(mut query : Query<&mut Speed, With<Player>>, keyboard_input: Res<Input<KeyCode>>) {
    const MAX_SPEED : f32 = 15.0;
    const MAX_BACK_SPEED : f32 = -15.0;
    const ACCELERATION : f32 = 5.0;

    for mut speed in query.iter_mut(){
        let to_positive = speed.x > 0.0;

        if keyboard_input.pressed(KeyCode::D){
            if to_positive{
                if speed.x < MAX_SPEED {
                    speed.x += ACCELERATION;
                }
            }else {
                if speed.x < MAX_SPEED {
                    speed.x += ACCELERATION * 2.0;
                }
            }
        }

        if keyboard_input.pressed(KeyCode::Q){
            if to_positive{
                if speed.x > MAX_BACK_SPEED {
                    speed.x -= ACCELERATION * 2.0;
                }
            }else {
                if speed.x > MAX_BACK_SPEED {
                    speed.x -= ACCELERATION;
                }
            }
        }

        println!("{:?}", speed);
    }
}

fn inertie(mut query : Query<&mut Speed>){
    const RESISTANCE : f32 = 2.0;

    for mut speed in query.iter_mut(){
        let to_positive = speed.x > 0.0;
        
        if to_positive{
            speed.x -= RESISTANCE;
            if speed.x < 0.0{
                speed.x = 0.0;
            }
        }else {
            speed.x += RESISTANCE;
            if speed.x > 0.0{
                speed.x = 0.0;
            } 
        }
    }
}
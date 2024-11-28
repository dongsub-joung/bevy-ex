// https://www.youtube.com/watch?v=B6ZFuYYZCSY&list=PL2wAo2qwCxGDp9fzBOTy_kpUTSwM1iWWd&index=1

use bevy::prelude::*;

#[derive(Component, Debug)]
struct Position{
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
struct Velocity{
    x: f32,
    y: f32,
}

fn main() {
    App::new()
        .add_systems(Startup, spawn_spaceship)
        .add_systems(Update, (update_position, print_position))
        .add_plugins(DefaultPlugins)
        .run();
}

fn spawn_spaceship(mut commands: Commands){
    commands.spawn((Position{ x: 0.0, y: 0.0}, Velocity { x: 1.0, y: 1.0 }) );
}

fn update_position(mut query: Query<(&Velocity, &mut Position)>){
    for (velocity, mut position) in query.iter_mut(){
        position.x+= velocity.x;
        position.y+= velocity.y;
    }
}

fn print_position(query: Query<(Entity, &Position)>){
    for (entity, position) in query.iter() {
        info!("Entitiy {:?} is at position {:?}", entity, position);
    }
}
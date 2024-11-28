use bevy::{input, prelude::*};

fn main() {
    App::new()
    // https://stackoverflow.com/questions/74898186/how-to-use-windowdescriptor-in-bevy
        .add_plugins(WindowPlugin {
            primary_window: Some(Window {
              resolution: (140.0, 140.0).into(),
              title: "Game of Life".to_string(),
              ..default()
            }),
            ..default()
          })
        .add_systems(Update, hello_world_system)
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(commands: &mut Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn hello_world_system() {
    println!("hello world");
 }
use bevy::{color::palettes::css::ORANGE_RED, prelude::*};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, generate_bodies)
        .add_systems(FixedUpdate, (interact_bodies, integrate))
        .add_systems(Update, look_at_star)
        .run();
}

const GRAVITY_CONSTANT: f32= 0.001;
const NUM_BODIES: usize= 100;

#[derive(Component, Default)]
struct Mass(f32);

#[derive(Component, Default)]
struct Acceleration(Vec3);

#[derive(Component, Default)]
struct LastPos(Vec3);

#[derive(Component)]
struct Start;

#[derive(Bundle, Default)]
struct BodyBundle{
    pbr: PbrBundle,
    mass: Mass,
    last_post: LastPos,
    acceleration: Acceleration,
}

fn generate_bodies(
    time: Res<Time<Fixed>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    let mesh= meshes.add(Sphere::new(1.0).mesh().ico(3).unwrap());
    
    let color_rnage= 0.5..1.0;
    let vel_range= -0.5..0.5;

    let mut rng = ChaCha8Rng::seed_from_u64(19878367467713);
    
    // https://bevyengine.org/examples/ecs-entity-component-system/iter-combinations/
}

// (interact_bodies, integrate)

// look_at_star
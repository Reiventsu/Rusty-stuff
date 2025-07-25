use bevy::asset::{Assets, Handle};
use bevy::color::Color;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{FromWorld, Mesh, Resource, Sphere, World};
use rand::SeedableRng;
use rand::seq::IndexedRandom;

#[derive(Resource)]
pub struct Power {
    pub charging: bool,
    pub current: f32,
}

#[derive(Resource)]
pub struct BallData {
    pub(crate) mesh: Handle<Mesh>,
    pub(crate) materials: Vec<Handle<StandardMaterial>>,
    pub(crate) rng: std::sync::Mutex<rand::rngs::StdRng>,
}
impl BallData {
    pub(crate) fn mesh(&self) -> Handle<Mesh> {
        self.mesh.clone()
    }
    pub(crate) fn material(&self) -> Handle<StandardMaterial> {
        let mut rng = self.rng.lock().unwrap();
        self.materials.choose(&mut *rng).unwrap().clone()
    }
}

impl FromWorld for BallData {
    fn from_world(world: &mut World) -> Self {
        let mesh = world.resource_mut::<Assets<Mesh>>().add(Sphere::new(1.));
        let mut materials = Vec::new();
        let mut material_assets = world.resource_mut::<Assets<StandardMaterial>>();
        for i in 0..36 {
            let color = Color::hsl((i * 10) as f32, 1., 0.5);
            materials.push(material_assets.add(StandardMaterial {
                base_color: color,
                ..Default::default()
            }));
        }
        let seed = *b"PhaestusFoxBevyBasicsRemastered0";
        BallData {
            mesh,
            materials,
            rng: std::sync::Mutex::new(rand::rngs::StdRng::from_seed(seed)),
        }
    }
}

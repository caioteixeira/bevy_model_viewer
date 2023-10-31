use bevy::{
    ecs::query,
    gltf::Gltf,
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
};

#[derive(Resource)]
struct ModelToSpawn(Handle<Gltf>);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(0.7, 0.7, 1.0).looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
        ..default()
    },));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 1,
            maximum_distance: 1.6,
            ..default()
        }
        .into(),
        ..default()
    });

    info!("Loading gltf objects");

    let gltf: Handle<Gltf> = asset_server.load("models/FlightHelmet/FlightHelmet.gltf");
    commands.insert_resource(ModelToSpawn(gltf));
}

fn spawn_gltf_objects(
    mut commands: Commands,
    model_to_load: Option<Res<ModelToSpawn>>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    if model_to_load.is_none() {
        return;
    }

    info!("Spawning gltf objects");

    if let Some(gltf) = assets_gltf.get(&model_to_load.unwrap().0) {
        for scene in gltf.scenes.iter() {
            commands.spawn(SceneBundle {
                scene: scene.clone(),
                ..default()
            });
        }

        commands.remove_resource::<ModelToSpawn>();
    }
}

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.3,
        })
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_gltf_objects)
        .run();
}

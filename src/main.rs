use std::fs;

use bevy::{
    gltf::Gltf,
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
    scene::SceneInstance,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_flycam::prelude::*;
use walkdir::WalkDir;

#[derive(Resource)]
struct ModelToSpawn(Handle<Gltf>);

#[derive(Resource)]
struct ModelPaths {
    paths: Vec<String>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.7, 0.7, 1.0)
                .looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
            ..default()
        },
        FlyCam,
    ));

    commands.spawn(
        TextBundle::from_section(
            "WASD : Move\nSpace: Ascend\nLeft Shift: Descend\nEsc: Grab/release cursor",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Left)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        }),
    );

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

    let path = "models/AntiqueCamera/glTF/AntiqueCamera.gltf";
    info!("Loading {}", path);
    let gltf: Handle<Gltf> = asset_server.load(path);
    commands.insert_resource(ModelToSpawn(gltf));
}

fn populate_list_of_models(mut model_paths: ResMut<ModelPaths>) {
    info!("Loading list of models");
    let walkdir = WalkDir::new("assets/models");

    for entry in walkdir {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            if path.extension().unwrap() == "gltf" {
                info!("{}", path.display());
                model_paths.paths.push(
                    path.display()
                        .to_string()
                        .trim_start_matches("assets/")
                        .replace("\\", "/"),
                );
            }
        }
    }
}

fn show_list_of_models(
    mut contexts: EguiContexts,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    model_paths: Res<ModelPaths>,
) {
    egui::Window::new("Models").show(contexts.ctx_mut(), |ui| {
        ui.heading("Models");

        for path in model_paths.paths.iter() {
            if ui.button(path).clicked() {
                info!("Loading {}", path);
                let gltf: Handle<Gltf> = asset_server.load(path);
                commands.insert_resource(ModelToSpawn(gltf));
            }
        }
    });
}

fn spawn_gltf_objects(
    mut commands: Commands,
    model_to_load: Option<Res<ModelToSpawn>>,
    assets_gltf: Res<Assets<Gltf>>,
    mut query_scenes: Query<Entity, With<SceneInstance>>,
) {
    if model_to_load.is_none() {
        return;
    }

    info!("Spawning gltf objects");
    if let Some(gltf) = assets_gltf.get(&model_to_load.unwrap().0) {
        // Unload current scenes
        for scene in query_scenes.iter_mut() {
            info!("Despawning scene");
            commands.entity(scene).despawn_recursive();

            //TODO: Check how to unload assets of the previous
        }

        // Load new scenes
        for scene in gltf.scenes.iter() {
            info!("Spawning scene");

            commands.spawn(SceneBundle {
                scene: scene.clone(),
                ..default()
            });
        }

        commands.remove_resource::<ModelToSpawn>();
    }
}

fn update_light_settings(mut contexts: EguiContexts, mut ambient_light: ResMut<AmbientLight>) {
    egui::Window::new("Light").show(contexts.ctx_mut(), |ui| {
        ui.heading("Ambient Light");
        ui.add(egui::Slider::new(&mut ambient_light.brightness, 0.0..=1.0).text("brightness"));

        let [r, g, b, a] = ambient_light.color.as_rgba_f32();
        let mut egui_color: egui::Rgba = egui::Rgba::from_srgba_unmultiplied(
            (r * 255.0) as u8,
            (g * 255.0) as u8,
            (b * 255.0) as u8,
            (a * 255.0) as u8,
        );

        if egui::widgets::color_picker::color_edit_button_rgba(
            ui,
            &mut egui_color,
            egui::color_picker::Alpha::Opaque,
        )
        .changed()
        {
            let [r, g, b, a] = egui_color.to_srgba_unmultiplied();
            ambient_light.color = [
                r as f32 / 255.0,
                g as f32 / 255.0,
                b as f32 / 255.0,
                a as f32 / 255.0,
            ]
            .into();
        }
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(NoCameraPlayerPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 7.0,           // default: 12.0
        })
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.3,
        })
        .insert_resource(ModelPaths { paths: Vec::new() })
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_systems(Startup, setup)
        .add_systems(Startup, populate_list_of_models)
        .add_systems(Update, show_list_of_models)
        .add_systems(Update, spawn_gltf_objects)
        .add_systems(Update, update_light_settings)
        .run();
}

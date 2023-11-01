use bevy::{
    gltf::Gltf,
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_flycam::prelude::*;

#[derive(Resource)]
struct ModelToSpawn(Handle<Gltf>);

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
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_gltf_objects)
        .add_systems(Update, update_light_settings)
        .run();
}

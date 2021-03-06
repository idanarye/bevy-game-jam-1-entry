// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::{App, ClearColor, Color, Msaa, WindowDescriptor};
use bevy::DefaultPlugins;
use bevy_egui_kbgp::{KbgpPlugin, KbgpSettings, KbgpNavBindings};
use bevy_rapier2d::physics::{NoUserData, RapierPhysicsPlugin};
use pogo_hurdling::GamePlugin;

fn main() {
    let mut app = App::new();
    app.insert_resource(Msaa { samples: 1 });
    app.insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)));
    app.insert_resource(WindowDescriptor {
        width: 800.,
        height: 600.,
        title: "Pogo Hurdling".to_string(),
        ..Default::default()
    });
    app.add_plugins(DefaultPlugins);
    app.add_plugin(GamePlugin);
    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
    app.add_plugin(benimator::AnimationPlugin::default());
    app.add_plugin(bevy_egui_kbgp::bevy_egui::EguiPlugin);
    app.insert_resource(bevy_egui_kbgp::bevy_egui::EguiSettings { scale_factor: 2.0 });
    app.add_plugin(KbgpPlugin);
    app.insert_resource(KbgpSettings {
        allow_keyboard: true,
        allow_mouse_buttons: true,
        allow_mouse_wheel: true,
        allow_mouse_wheel_sideways: true,
        allow_gamepads: true,
        bindings: KbgpNavBindings::default().with_wasd_navigation(),
    });
    app.run();
}

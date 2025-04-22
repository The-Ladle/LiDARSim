//! Game project.
pub mod generate_sensor;
pub mod cast_rays;
pub mod add_sensors;

use fyrox::{
    core::pool::Handle, core::visitor::prelude::*, core::reflect::prelude::*,
    event::Event,
    gui::message::UiMessage,
    plugin::{Plugin, PluginContext, PluginRegistrationContext},
    scene::Scene,
};
use generate_sensor::GenerateSensor;
use std::path::Path;

// Re-export the engine.
pub use fyrox;
use crate::add_sensors::AddSensors;
use crate::cast_rays::CastRays;

#[derive(Default, Visit, Reflect, Debug)]
pub struct Game {
    scene: Handle<Scene>,
}

impl Plugin for Game {
    fn register(&self, _context: PluginRegistrationContext) {
        // Register your scripts here.
        _context
            .serialization_context
            .script_constructors
            .add::<GenerateSensor>("Generate Sensor")
            .add::<CastRays>("Cast Rays")
            .add::<AddSensors>("Add Sensors");
    }
    
    fn init(&mut self, scene_path: Option<&str>, context: PluginContext) {
        context
            .async_scene_loader
            .request(scene_path.unwrap_or("data/scene.rgs"));
    }

    fn on_deinit(&mut self, _context: PluginContext) {
        // Do a cleanup here.
    }

    fn update(&mut self, _context: &mut PluginContext) {
        // Add your global update code here.
    }

    fn on_os_event(
        &mut self,
        _event: &Event<()>,
        _context: PluginContext,
    ) {
        // Do something on OS event here.
    }

    fn on_ui_message(
        &mut self,
        _context: &mut PluginContext,
        _message: &UiMessage,
    ) {
        // Handle UI events here.
    }

    fn on_scene_begin_loading(&mut self, _path: &Path, ctx: &mut PluginContext) {
        if self.scene.is_some() {
            ctx.scenes.remove(self.scene);
        }
    }

    fn on_scene_loaded(
        &mut self,
        _path: &Path,
        scene: Handle<Scene>,
        _data: &[u8],
        _context: &mut PluginContext,
    ) {
        self.scene = scene;
    }
}

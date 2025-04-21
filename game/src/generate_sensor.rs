use std::future::Future;
use std::path::Path;

use fyrox::{
    asset::manager::ResourceManager, core::{algebra::{Matrix, Quaternion, Unit, UnitQuaternion, Vector3}, pool::Handle, reflect::prelude::*, type_traits::prelude::*, visitor::prelude::*}, event::Event, resource::model::{Model, ModelResourceExtension}, scene::{node::Node, Scene}, script::{ScriptContext, ScriptDeinitContext, ScriptTrait}
};
use fyrox::core::futures;
use fyrox::core::futures::executor::block_on;

#[derive(Visit, Reflect, Default, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "c9a034a0-87ab-43f8-8f50-64495a3964ed")]
#[visit(optional)]
pub struct GenerateSensor {
    // Add fields here.
    sensor_width_px: u16,
    sensor_height_px: u16,
    sensor_width: f32,
    sensor_height: f32,
    sensor_fov_diagonal: f32,
    pixel_prefab_path: String,
}

impl ScriptTrait for GenerateSensor {
    
    
    fn on_init(&mut self, context: &mut ScriptContext) {
        // Put initialization logic here.

    }

    fn on_start(&mut self, context: &mut ScriptContext) {
        // There should be a logic that depends on other scripts in scene.
        // It is called right after **all** scripts were initialized.
        let sensor_fov_horizontal = get_fov_horizontal(self.sensor_width_px, self.sensor_height_px, self.sensor_fov_diagonal);
        let sensor_fov_vertical = get_fov_vertical(self.sensor_width_px, self.sensor_height_px, self.sensor_fov_diagonal);
        let path = Path::new(&self.pixel_prefab_path);
        let node_mut = context.handle;
        let mut i: u16 = 0;
        while i < self.sensor_width_px {
            let mut j: u16 = 0;
            while j < self.sensor_height_px {
                block_on(instantiate_model(node_mut, path, context.resource_manager, context.scene, i, j, self.sensor_width, self.sensor_height, self.sensor_width_px, self.sensor_height_px, sensor_fov_horizontal, sensor_fov_vertical));
                j = j+1;
            }
            i = i+1;
        }
    }

    fn on_deinit(&mut self, context: &mut ScriptDeinitContext) {
        // Put de-initialization logic here.
    }

    fn on_os_event(&mut self, event: &Event<()>, context: &mut ScriptContext) {
        // Respond to OS events here.
    }

    fn on_update(&mut self, context: &mut ScriptContext) {
        // Put object logic here.
    }
    
    fn on_message(
        &mut self,
        #[allow(unused_variables)] message: &mut dyn fyrox::script::ScriptMessagePayload,
        #[allow(unused_variables)] ctx: &mut fyrox::script::ScriptMessageContext,
    ) {
    }
}

fn get_fov_horizontal(w: u16, h: u16, dfov: f32) -> f32{
    return ((dfov/2f32).tan() * (h/(h*h + w*w).isqrt()) as f32).atan() * 2f32;
}

fn get_fov_vertical(w: u16, h: u16, dfov: f32) -> f32{
    return ((dfov/2f32).tan() * (w/(h*h + w*w).isqrt()) as f32).atan() * 2f32;
}

async fn instantiate_model (
    parent_node: Handle<Node>,
    path: &Path,
    resource_manager: &ResourceManager,
    scene: &mut Scene,
    i: u16,
    j: u16,
    sensor_width: f32,
    sensor_height: f32,
    sensor_width_px: u16,
    sensor_height_px: u16,
    horizontal_fov: f32,
    vertical_fov: f32,
) -> Handle<Node>{
    // Load model first. Alternatively, you can store resource handle somewhere and use it for
    // instantiation.
    let model = resource_manager.request::<Model>(path).await.unwrap();

    let position: Vector3<f32> = Vector3::new(
        (i as f32* sensor_width /(sensor_width_px - 1) as f32) - sensor_width /2f32,
        (-(j as f32)* sensor_height /(sensor_height_px - 1) as f32) + sensor_height /2f32,
        0f32
    );
    let pitch = j as f32* vertical_fov /(sensor_height_px - 1) as f32 - vertical_fov /2f32;
    let yaw = i as f32* horizontal_fov /(sensor_width_px -1) as f32 - horizontal_fov /2f32;

    return model.instantiate_and_attach(
        scene,
        parent_node,
        position,
        euler_to_look_direction(position, yaw, pitch),
        Vector3::new(1f32,1f32,1f32)
    );
}

pub fn euler_to_look_direction(position: Vector3<f32>, yaw: f32, pitch: f32) -> Vector3<f32> {
    // Convert degrees to radians.
    let yaw_rad = yaw.to_radians();
    let pitch_rad = pitch.to_radians();

    // Calculate the direction vector.
    // Note: For a left-handed system similar to Unity,
    // - Yaw (rotation around Y) affects X and Z.
    // - Pitch (rotation around X) affects Y.
    let x = pitch_rad.cos() * yaw_rad.sin();
    let y = pitch_rad.sin();
    let z = pitch_rad.cos() * yaw_rad.cos();

    let pre_offset: Vector3<f32> = Vector3::new(x, y, z).normalize();
    return pre_offset + position;
}
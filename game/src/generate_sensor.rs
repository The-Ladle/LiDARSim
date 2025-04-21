
use std::path::Path;

use fyrox::{
    asset::manager::ResourceManager, core::{algebra::{Matrix, Quaternion, Unit, UnitQuaternion, Vector3}, pool::Handle, reflect::prelude::*, type_traits::prelude::*, visitor::prelude::*}, event::Event, resource::model::{Model, ModelResourceExtension}, scene::{node::Node, Scene}, script::{ScriptContext, ScriptDeinitContext, ScriptTrait}
};

#[derive(Visit, Reflect, Default, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "c9a034a0-87ab-43f8-8f50-64495a3964ed")]
#[visit(optional)]
pub struct GenerateSensor {
    // Add fields here.
    sensorWidthPx: u16,
    sensorHeightPx: u16,
    sensorWidth: f32,
    sensorHeight: f32,
    sensorFovDiagonal: f32,
    pixelPrefabPath: String,
}

impl ScriptTrait for GenerateSensor {
    
    
    fn on_init(&mut self, context: &mut ScriptContext) {
        // Put initialization logic here.
        let sensorFovHorizontal = getFovHorizontal(self.sensorWidthPx, self.sensorHeightPx, self.sensorFovDiagonal);
        let sensorFovVertical = getFovVertical(self.sensorWidthPx, self.sensorHeightPx, self.sensorFovDiagonal);
        let path = Path::new(&self.pixelPrefabPath);
        let mut i: u16 = 0;
        while i < self.sensorHeightPx{
            let mut j: u16 = 0;
            while j < self.sensorHeightPx{
                instantiate_model(path, context.resource_manager, context.scene, i, j, self.sensorWidth, self.sensorHeight, self.sensorWidthPx, self.sensorHeightPx, sensorFovHorizontal, sensorFovVertical);
                j = j+1;
            }
            i = i+1;
        }
    }

    fn on_start(&mut self, context: &mut ScriptContext) {
        // There should be a logic that depends on other scripts in scene.
        // It is called right after **all** scripts were initialized.
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

fn getFovHorizontal(w: u16, h: u16, dfov: f32) -> f32{
    return ((dfov/2f32).tan() * (h/(h*h + w*w).isqrt()) as f32).atan() * 2f32;
}

fn getFovVertical(w: u16, h: u16, dfov: f32) -> f32{
    return ((dfov/2f32).tan() * (w/(h*h + w*w).isqrt()) as f32).atan() * 2f32;
}

async fn instantiate_model (
    path: &Path,
    resource_manager: &ResourceManager,
    scene: &mut Scene,
    i: u16,
    j: u16,
    sensorWidth: f32,
    sensorHeight: f32,
    sensorWidthPx: u16,
    sensorHeightPx: u16,
    horizontalFov: f32,
    verticalFov: f32,
) {
    // Load model first. Alternatively, you can store resource handle somewhere and use it for
    // instantiation.
    let model = resource_manager.request::<Model>(path).await.unwrap();

    let node = model.instantiate(scene);

    scene.graph[node]
                .local_transform_mut()
                .set_position(Vector3::new((i as f32*sensorWidth/(sensorWidthPx - 1) as f32) - sensorWidth/2f32, (-(j as f32)*sensorHeight/(sensorHeightPx - 1) as f32) + sensorHeight/2f32, 0f32))
                .set_rotation(UnitQuaternion::from_euler_angles(0f32, j as f32*verticalFov/(sensorHeightPx - 1) as f32 - verticalFov/2f32, i as f32*horizontalFov/(sensorWidthPx-1) as f32 - horizontalFov/2f32));
}
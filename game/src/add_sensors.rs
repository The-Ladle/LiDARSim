use std::path::Path;
use fyrox::{
    core::{visitor::prelude::*, reflect::prelude::*, type_traits::prelude::*},
    event::Event, script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};
use fyrox::asset::manager::ResourceManager;
use fyrox::core::algebra::Vector3;
use fyrox::core::futures::executor::block_on;
use fyrox::core::pool::Handle;
use fyrox::resource::model::{Model, ModelResourceExtension};
use fyrox::scene::node::Node;
use fyrox::scene::Scene;
use fyrox::script::{ScriptMessageContext, ScriptMessagePayload};
use crate::generate_sensor::euler_to_look_direction;
use crate::manage_commands::Message;

#[derive(Visit, Reflect, Default, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "99d3ade3-b39c-404b-990b-c10d84bece7e")]
#[visit(optional)]
pub struct AddSensors {
    // Add fields here.
    sensor_path: String,
}

impl ScriptTrait for AddSensors {
    fn on_start(&mut self, ctx: &mut ScriptContext) {
        ctx.message_dispatcher.subscribe_to::<Message>(ctx.handle);
    }
    fn on_message(&mut self, message: &mut dyn ScriptMessagePayload, ctx: &mut ScriptMessageContext) {
        if let Some(Message::AddSensors { sensors }) = message.downcast_ref::<Message>(){
            for sensor in sensors{
                block_on(instantiate_model(ctx.handle, Path::new(&self.sensor_path), ctx.resource_manager, ctx.scene, sensor));
            }
        }
    }
}

async fn instantiate_model (
    parent_node: Handle<Node>,
    path: &Path,
    resource_manager: &ResourceManager,
    scene: &mut Scene,
    position: &Vector3<f32>
) -> Handle<Node>{
    // Load model first. Alternatively, you can store resource handle somewhere and use it for
    // instantiation.
    let model = resource_manager.request::<Model>(path).await.unwrap();

    return model.instantiate_and_attach(
        scene,
        parent_node,
        *position,
        euler_to_look_direction(*position, 0.0, 0.0),
        Vector3::new(1f32,1f32,1f32)
    );
}
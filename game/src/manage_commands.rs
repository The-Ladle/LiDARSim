#[derive(Debug)]
pub enum Message {
    AddSensors{
        sensors: Vec<Vector3<f32>>
    },
    RaycastStateChange{
        on: bool
    },
    SpawnField{
    }
}

use fyrox::{
    core::{visitor::prelude::*, reflect::prelude::*, type_traits::prelude::*},
    event::Event, script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};
use fyrox::core::algebra::{Vector, Vector3};
use fyrox::core::math::Matrix4Ext;
use fyrox::core::pool::Handle;
use fyrox::graph::{AbstractSceneGraph, BaseSceneGraph};
use fyrox::scene::node::Node;

#[derive(Visit, Reflect, Default, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "2b823c01-691a-4617-8fb1-fa52cf95155d")]
#[visit(optional)]
pub struct ManageCommands {
    // Add fields here.
}

impl ScriptTrait for ManageCommands {
    fn on_update(&mut self, ctx: &mut ScriptContext) {
        
    }
    fn on_start(&mut self, ctx: &mut ScriptContext) {
        //ctx.message_sender.send_global(Message::SpawnField {});
        let mut position: Vector3<f32> = Vector3::new(0f32, 0f32, 0f32);
        if let Some(node) = ctx.scene.graph.try_get(ctx.handle){
            let global_transform = node.global_transform();

            position = global_transform.position();
        }
        if(true) {
            ctx.message_sender.send_global(Message::AddSensors {
                sensors: vec![position]
            });
        }
        ctx.message_sender.send_global(Message::RaycastStateChange {
            on: true
        })
    }
}
    
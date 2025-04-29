#[derive(Debug)]
pub enum Message {
    AddSensors{
        sensors: Vec<Vector3<f32>>
    },
    RaycastStateChange{
        on: bool
    }
}

use fyrox::{
    core::{visitor::prelude::*, reflect::prelude::*, type_traits::prelude::*},
    event::Event, script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};
use fyrox::core::algebra::Vector3;
use fyrox::core::pool::Handle;
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
        if(true) {
            ctx.message_sender.send_global(Message::AddSensors {
                sensors: vec![Vector3::new(0.0, 0.0, 0.0)],
            });
        }
        ctx.message_sender.send_global(Message::RaycastStateChange {
            on: true
        })
    }
}
    
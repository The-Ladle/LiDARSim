
use fyrox::{
    core::{visitor::prelude::*, reflect::prelude::*, type_traits::prelude::*},
    event::Event, script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};
use fyrox::core::algebra::{Point3, Vector3};
use fyrox::core::arrayvec::ArrayVec;
use fyrox::core::pool::Handle;
use fyrox::scene::graph::Graph;
use fyrox::scene::graph::physics::{RayCastOptions, Intersection};
use fyrox::scene::node::Node;

#[derive(Visit, Reflect, Default, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "eae25d70-49bc-4779-951a-20e35e143c28")]
#[visit(optional)]
pub struct CastRays {
    // Add fields here.
}

impl ScriptTrait for CastRays {
    fn on_init(&mut self, context: &mut ScriptContext) {
        // Put initialization logic here.
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
        let handle = context.handle;
        let graph_ptr = &mut context.scene.graph as *mut Graph; // raw pointer escape hatch

        // SAFETY: We are not mutably using the graph before this call.
        let node = unsafe { &(*graph_ptr)[handle] };

        lidar_raycast(unsafe { &mut *graph_ptr }, node);
    }

}

fn do_static_ray_cast<const N: usize>(
    graph: &mut Graph,
    node: &Node,
) -> ArrayVec<Intersection, N> {
    let mut buffer = ArrayVec::<Intersection, N>::new();

    graph.physics.cast_ray(
        RayCastOptions {
            ray_origin: node.global_position().into(),
            ray_direction: node.look_vector(),
            max_len: 1000.0,
            groups: Default::default(),
            sort_results: true,
        },
        &mut buffer
    );

    buffer
}

fn lidar_raycast(graph: &mut Graph, node: &Node){
    // Fetch first 32 intersections.
    dbg!(do_static_ray_cast::<1>(graph, node));
}

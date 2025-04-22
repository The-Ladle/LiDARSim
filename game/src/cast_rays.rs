
use fyrox::{
    core::{visitor::prelude::*, reflect::prelude::*, type_traits::prelude::*},
    event::Event, script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};
use fyrox::core::algebra::{Point3, Vector3};
use fyrox::core::arrayvec::ArrayVec;
use fyrox::core::pool::Handle;
use fyrox::core::variable::InheritableVariable;
use fyrox::scene::graph::Graph;
use fyrox::scene::graph::physics::{RayCastOptions, Intersection};
use fyrox::scene::node::Node;

#[derive(Visit, Reflect, Default, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "eae25d70-49bc-4779-951a-20e35e143c28")]
#[visit(optional)]
pub struct CastRays {
    // Add fields here.
    trail: InheritableVariable<Handle<Node>>,
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
        /*let handle = context.handle;
        let graph_ptr = &mut context.scene.graph as *mut Graph; // raw pointer escape hatch

        // SAFETY: We are not mutably using the graph before this call.
        let node = unsafe { &(*graph_ptr)[handle] };

        lidar_raycast(unsafe { &mut *graph_ptr }, node);*/
        let this_node = &context.scene.graph[context.handle];
        let this_node_position = this_node.global_position();

        // Cast a ray in from the node in its "look" direction.
        let mut intersections = Vec::new();
        context.scene.graph.physics.cast_ray(
            RayCastOptions {
                ray_origin: this_node_position.into(),
                ray_direction: this_node.look_vector(),
                max_len: 3.0,
                groups: Default::default(),
                // Sort results of the ray casting so the closest intersection will be in the
                // beginning of the list.
                sort_results: true,
            },
            &mut intersections,
        );

        let trail_length = if let Some(intersection) = intersections.first() {
            // If we got an intersection, scale the trail by the distance between the position of the node
            // with this script and the intersection position.
            this_node_position.metric_distance(&intersection.position.coords)
        } else {
            // Otherwise the trail will be as large as possible.
            1000.0
        };

        if let Some(trail_node) = context.scene.graph.try_get_mut(*self.trail) {
            let transform = trail_node.local_transform_mut();
            let current_trail_scale = **transform.scale();
            transform.set_scale(Vector3::new(
                // Keep x scaling.
                current_trail_scale.x,
                trail_length,
                // Keep z scaling.
                current_trail_scale.z,
            ));
        }
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

use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::mpsc::{self, Receiver};
use std::thread;

use network_tables::v4::subscription::SubscriptionOptions;
use tokio::stream::StreamExt; // for subscription.next().await

// Fyrox engine imports.
use fyrox::{
    core::algebra::Vector3,
    scene::node::Node,
    script::{Script, ScriptContext},
};
use fyrox::core::reflect::Reflect;
use fyrox::core::{ComponentProvider, TypeUuidProvider};
use fyrox::core::visitor::Visit;
use fyrox::script::ScriptTrait;

// Helper function that parses a comma-separated string like "x,y" into a (f32, f32) tuple.
fn parse_position(s: &str) -> Option<(f32, f32)> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 2 {
        return None;
    }
    let x = parts[0].trim().parse::<f32>().ok()?;
    let y = parts[1].trim().parse::<f32>().ok()?;
    Some((x, y))
}

#[derive(Visit, Reflect, Default, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "2b823c01-691a-4617-8fb1-fa52cf95155d")]
#[visit(optional)]
pub struct MoveRobot {
    // This receiver is not serialized so mark with serde(skip)
    #[serde(skip)]
    rx: Option<Receiver<(f32, f32)>>,
}

impl ScriptTrait for MoveRobot {
    // Called once when the script is attached.
    fn on_start(&mut self, _context: &ScriptContext) {
        // Only initialize once.
        if self.rx.is_none() {
            // Create an mpsc channel to receive (x, y) updates.
            let (tx, rx) = mpsc::channel::<(f32, f32)>();
            self.rx = Some(rx);

            // Spawn a separate thread with its own Tokio runtime to handle network I/O.
            thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
                rt.block_on(async move {
                    // Connect to NetworkTables; adjust the IP and port as needed.
                    let client = network_tables::v4::Client::try_new_w_config(
                        SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 5810),
                        network_tables::v4::client_config::Config::default(),
                    )
                        .await
                        .expect("Failed to connect to NetworkTables");

                    // Subscribe to the topic that contains the position.
                    // We assume the value is a string like "100.0,200.0".
                    let mut subscription = client
                        .subscribe_w_options(
                            &["/AdvantageKit/RealOutputs/Odometry/Robot"],
                            Some(SubscriptionOptions {
                                all: Some(true),
                                prefix: Some(false),
                                ..Default::default()
                            }),
                        )
                        .await
                        .expect("Failed to subscribe to topic");

                    // Process incoming messages.
                    while let Some(message) = subscription.next().await {
                        if let network_tables::Value::String(ref s) = message {
                            if let Some((x, y)) = parse_position(s) {
                                // Send the new position over the channel.
                                let _ = tx.send((x, y));
                            }
                        }
                    }
                });
            });
        }
    }

    // Called every frame.
    fn on_update(&mut self, _context: &ScriptContext) {
        // Check if we have a valid channel.
        if let Some(rx) = &self.rx {
            // Process all pending position updates (if any).
            while let Ok((x, y)) = rx.try_recv() {
                // Update the owner's local position; Fyrox uses Vector3 so we set z to 0.
                owner.local_transform_mut().set_position(Vector3::new(x, y, 0.0));
            }
        }
    }
}

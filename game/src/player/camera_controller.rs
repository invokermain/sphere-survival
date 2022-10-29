use fyrox::{
    core::{
        algebra::{UnitQuaternion, Vector3},
        pool::Handle,
    },
    engine::resource_manager::ResourceManager,
    event::DeviceEvent,
    scene::{
        base::BaseBuilder,
        camera::CameraBuilder,
        graph::Graph,
        node::Node,
        pivot::{Pivot, PivotBuilder},
        transform::{Transform, TransformBuilder},
    },
};

// Camera controller consists of three scene nodes - two pivots and one camera.
pub struct CameraController {
    // Pivot is the origin of our camera controller.
    pub pivot: Handle<Node>,
    camera: Handle<Node>,
    // An angle around local Y axis of the pivot.
    pub yaw: f32,
    // An angle around local X axis of the hinge.
    pitch: f32,
}

impl CameraController {
    pub fn new(graph: &mut Graph) -> Self {
        let camera;
        let pivot = PivotBuilder::new(BaseBuilder::new().with_children(&[{
            camera = CameraBuilder::new(
                BaseBuilder::new().with_local_transform(
                    TransformBuilder::new()
                        .with_local_position(Vector3::new(0.0, 0.0, -2.0))
                        .build(),
                ),
            )
            .with_z_far(48.0)
            .build(graph);
            camera
        }]))
        .build(graph);

        Self {
            pivot,
            camera,
            yaw: 0.0,
            pitch: 0.0,
        }
    }

    pub fn handle_device_event(&mut self, device_event: &DeviceEvent) {
        if let DeviceEvent::MouseMotion { delta } = device_event {
            const MOUSE_SENSITIVITY: f32 = 0.0025;

            self.yaw -= (delta.0 as f32) * MOUSE_SENSITIVITY;
            self.pitch = (self.pitch + (delta.1 as f32) * MOUSE_SENSITIVITY)
                // Limit vertical angle to [-90; 90] degrees range
                .max(-90.0f32.to_radians())
                .min(90.0f32.to_radians());
        }
    }

    pub fn update(&mut self, graph: &mut Graph) {
        // Apply rotation to the pivot.
        let transform: &mut Transform = graph[self.pivot].local_transform_mut();

        transform.set_rotation(
            UnitQuaternion::from_axis_angle(&Vector3::y_axis(), self.yaw)
                * UnitQuaternion::from_axis_angle(&Vector3::x_axis(), self.pitch),
        );
    }
}

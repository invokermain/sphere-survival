mod camera_controller;
pub use camera_controller::CameraController;
use fyrox::{
    core::{
        algebra::{Quaternion, Unit, Vector3, Vector4, Vector6},
        pool::Handle,
        profiler::print,
    },
    event::{DeviceEvent, ElementState, KeyboardInput, VirtualKeyCode},
    scene::{
        base::BaseBuilder,
        collider::{ColliderBuilder, ColliderShape},
        node::Node,
        rigidbody::RigidBodyBuilder,
        transform::{Transform, TransformBuilder},
        Scene,
    },
};

#[derive(Default)]
struct Thrust {
    forward: bool,
    back: bool,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl Thrust {
    fn to_vector(&self) -> Vector3<f32> {
        Vector3::new(
            (self.left as i8 - self.right as i8) as f32,
            (self.up as i8 - self.down as i8) as f32,
            (self.forward as i8 - self.back as i8) as f32,
        )
    }

    fn to_rotated_vector(&self, rotation: Unit<Quaternion<f32>>) -> Vector3<f32> {
        let r_vector = rotation.vector();
        let r_scalar = rotation.scalar();

        let thrust_vector = self.to_vector();

        thrust_vector * (r_scalar.powi(2) - r_vector.magnitude_squared())
            + r_vector * thrust_vector.dot(&r_vector) * 2.0
            + r_vector.cross(&thrust_vector) * r_scalar * 2.0
    }
}

pub struct Player {
    camera_controller: CameraController,
    body: Handle<Node>,
    collider: Handle<Node>,
    thrust: Thrust,
    momentum: Vector3<f32>,
}

impl Player {
    pub fn new(scene: &mut Scene) -> Self {
        // Create new rigid body and offset it a bit to prevent falling through the ground.
        let collider;
        let body = RigidBodyBuilder::new(
            BaseBuilder::new()
                .with_local_transform(
                    TransformBuilder::new()
                        .with_local_position(Vector3::new(0.0, 0.0, 0.0))
                        .build(),
                )
                .with_children(&[{
                    // Create capsule collider with friction disabled. We need to disable friction because linear
                    // velocity will be set manually, but the physics engine will reduce it using friction so it
                    // won't let us to set linear velocity precisely.
                    collider = ColliderBuilder::new(BaseBuilder::new())
                        .with_shape(ColliderShape::capsule_y(0.55, 0.15))
                        .with_friction(0.0)
                        .build(&mut scene.graph);
                    collider
                }]),
        )
        .with_can_sleep(false)
        .with_gravity_scale(0.0)
        .build(&mut scene.graph);

        Self {
            // As a final stage create camera controller.
            camera_controller: CameraController::new(&mut scene.graph),
            collider,
            body,
            thrust: Default::default(),
            momentum: Default::default(),
        }
    }

    pub fn handle_device_event(&mut self, device_event: &DeviceEvent) {
        self.camera_controller.handle_device_event(device_event)
    }

    pub fn handle_key_event(&mut self, key: &KeyboardInput) {
        if let Some(key_code) = key.virtual_keycode {
            match key_code {
                VirtualKeyCode::W => self.thrust.forward = key.state == ElementState::Pressed,
                VirtualKeyCode::S => self.thrust.back = key.state == ElementState::Pressed,
                VirtualKeyCode::A => self.thrust.left = key.state == ElementState::Pressed,
                VirtualKeyCode::D => self.thrust.right = key.state == ElementState::Pressed,
                VirtualKeyCode::LShift => self.thrust.up = key.state == ElementState::Pressed,
                VirtualKeyCode::LControl => self.thrust.down = key.state == ElementState::Pressed,
                _ => (),
            }
        }
    }

    pub fn update(&mut self, scene: &mut Scene, dt: f32) {
        self.camera_controller.update(&mut scene.graph);

        let camera_transform: &Transform =
            scene.graph[self.camera_controller.pivot].local_transform();

        self.momentum += self.thrust.to_rotated_vector(**camera_transform.rotation()) * dt * 0.25;

        let body_transform: &mut Transform = scene.graph[self.body].local_transform_mut();
        body_transform.set_position(**body_transform.position() + self.momentum);
        let new_position = **body_transform.position();

        let camera_transform: &mut Transform =
            scene.graph[self.camera_controller.pivot].local_transform_mut();
        camera_transform.set_position(new_position);
    }
}

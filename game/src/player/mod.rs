mod camera_controller;
pub use camera_controller::CameraController;
use fyrox::{
    core::{
        algebra::{Quaternion, Unit, Vector3},
        pool::Handle,
    },
    event::{DeviceEvent, ElementState, KeyboardInput, VirtualKeyCode},
    scene::{
        base::BaseBuilder,
        collider::{Collider, ColliderBuilder, ColliderShape},
        node::Node,
        rigidbody::{RigidBodyBuilder, RigidBodyType},
        transform::Transform,
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
    mouse_sensitivity: f32,
    camera_controller: CameraController,
    body: Handle<Node>,
    collider: Handle<Node>,
    thrust: Thrust,
    momentum: Vector3<f32>,
}

impl Player {
    pub fn new(scene: &mut Scene) -> Self {
        let collider;
        let body = RigidBodyBuilder::new(
            BaseBuilder::new()
                .with_name("PlayerRigidBody")
                .with_children(&[{
                    collider = ColliderBuilder::new(BaseBuilder::new().with_name("PlayerCollider"))
                        .with_shape(ColliderShape::ball(9.5))
                        .with_friction(0.0)
                        .build(&mut scene.graph);
                    collider
                }]),
        )
        .with_body_type(RigidBodyType::Dynamic)
        .with_gravity_scale(0.0)
        .with_ccd_enabled(true)
        .build(&mut scene.graph);

        Self {
            mouse_sensitivity: 0.0025,
            camera_controller: CameraController::new(&mut scene.graph),
            collider,
            body,
            thrust: Default::default(),
            momentum: Default::default(),
        }
    }

    pub fn handle_device_event(&mut self, device_event: &DeviceEvent) {
        if let DeviceEvent::MouseMotion { delta } = device_event {
            self.camera_controller.rotate_by(
                -1.0 * (delta.0 as f32) * self.mouse_sensitivity,
                (delta.1 as f32) * self.mouse_sensitivity,
                0.0,
            )
        }
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
        // update camera so rotation is up to date
        self.camera_controller.update(&mut scene.graph);

        // handle colliders events
        let collider = scene.graph[self.collider].as_collider();

        let collision_event = collider
            .contacts(&scene.graph.physics)
            .find(|f| f.has_any_active_contact);

        if let Some(collision_event) = collision_event {
            let collider1: &Collider = scene.graph[collision_event.collider1].as_collider();
            let collider2: &Collider = scene.graph[collision_event.collider2].as_collider();

            match collider1.name() {
                "WorldBoundsCollider" => {
                    // we are out of bounds need to bounce off of the wall
                    // get our body transform
                    let body_transform: &mut Transform =
                        scene.graph[self.body].local_transform_mut();
                    if self.momentum.dot(&**body_transform.position()) > 0.0 {
                        self.momentum *= -1.0;
                        self.apply_momentum(scene);
                    };
                    return;
                }
                _ => {
                    println!(
                        "collision: {:?}, {:?}, {}",
                        collider1.name(),
                        collider2.name(),
                        collision_event.has_any_active_contact
                    )
                }
            };
        }

        // get the camera's current rotation
        let rotation = self.camera_controller.get_rotation(&scene.graph);

        // update momentum by adding thrust in the direction we are looking
        self.momentum += self.thrust.to_rotated_vector(rotation) * dt * 0.25;

        self.apply_momentum(scene)
    }

    fn apply_momentum(&mut self, scene: &mut Scene) {
        // get our body transform
        let body_transform: &mut Transform = scene.graph[self.body].local_transform_mut();

        // calculate our new position
        let position = **body_transform.position() + self.momentum;

        // set our new position
        body_transform.set_position(position);

        // update the camera to be in the same position
        self.camera_controller
            .set_position(&mut scene.graph, position);
    }
}

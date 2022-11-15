use fyrox::{
    core::{
        algebra::{Quaternion, Unit, Vector3},
        num_traits::Zero,
        pool::Handle,
    },
    event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent},
    scene::{node::Node, rigidbody::RigidBody},
};

use fyrox::{
    core::{
        reflect::prelude::*,
        uuid::{uuid, Uuid},
        visitor::prelude::*,
    },
    engine::resource_manager::ResourceManager,
    event::Event,
    impl_component_provider,
    scene::node::TypeUuidProvider,
    script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};

#[derive(Default, Visit, Reflect, Debug, Clone)]
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

#[derive(Visit, Reflect, Default, Debug, Clone)]
pub struct Player {
    camera_pivot: Handle<Node>,
    body: Handle<Node>,
    collider: Handle<Node>,
    thrust: Thrust,
}

impl_component_provider!(Player);

impl TypeUuidProvider for Player {
    fn type_uuid() -> Uuid {
        uuid!("4b1e2f80-e916-4542-b18d-699bdb49a275")
    }
}

impl ScriptTrait for Player {
    fn on_init(&mut self, _context: &mut ScriptContext) {
        // Put initialization logic here.
    }

    fn on_start(&mut self, _context: &mut ScriptContext) {
        // There should be a logic that depends on other scripts in scene.
        // It is called right after **all** scripts were initialized.
    }

    fn on_deinit(&mut self, _context: &mut ScriptDeinitContext) {
        // Put de-initialization logic here.
    }

    fn on_os_event(&mut self, event: &Event<()>, _context: &mut ScriptContext) {
        if let Event::WindowEvent {
            event: WindowEvent::KeyboardInput { input, .. },
            ..
        } = event
        {
            self.handle_key_event(input);
        }
    }

    fn on_update(&mut self, context: &mut ScriptContext) {
        let scene = &mut context.scene;
        let dt = context.dt;

        // get the camera's current rotation
        let rotation = **scene.graph[self.camera_pivot].local_transform().rotation();
        // get our body
        let body: &mut RigidBody = scene.graph[self.body].as_rigid_body_mut();
        let current_velocity = body.lin_vel();

        // calculate thrust in the direction we are looking
        let thrust_vector = self.thrust.to_rotated_vector(rotation) * dt;

        if !thrust_vector.is_zero() {
            let mut new_velocity = current_velocity + thrust_vector * 150.0;
            if new_velocity.magnitude() > 150.0 {
                new_velocity = new_velocity.normalize() * 150.0;
            }
            body.set_lin_vel(new_velocity);
        } else if current_velocity.magnitude() > 25.0 {
            let new_velocity = current_velocity - current_velocity * 4.0 * dt;
            body.set_lin_vel(new_velocity);
        }
    }

    fn restore_resources(&mut self, _resource_manager: ResourceManager) {
        // Restore resource handles here.
    }

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}

impl Player {
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
}

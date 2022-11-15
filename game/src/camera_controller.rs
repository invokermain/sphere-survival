use fyrox::{
    core::{
        algebra::{UnitQuaternion, Vector3},
        pool::Handle,
        reflect::prelude::*,
        uuid::{uuid, Uuid},
        visitor::prelude::*,
    },
    engine::resource_manager::ResourceManager,
    event::{DeviceEvent, Event},
    impl_component_provider,
    scene::node::{Node, TypeUuidProvider},
    script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};

#[derive(Visit, Reflect, Default, Debug, Clone)]
pub struct CameraController {
    mouse_sensitivity: f32,
    pivot: Handle<Node>,
    camera: Handle<Node>,
    yaw: f32,
    pitch: f32,
    roll: f32,
}

impl_component_provider!(CameraController);

impl TypeUuidProvider for CameraController {
    fn type_uuid() -> Uuid {
        uuid!("8d20e159-c16e-4a99-9ada-8ee3df8b1758")
    }
}

impl ScriptTrait for CameraController {
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
        if let Event::DeviceEvent { event, .. } = event {
            self.handle_device_event(event)
        }
    }

    fn on_update(&mut self, context: &mut ScriptContext) {
        let rotation = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), self.yaw)
            * UnitQuaternion::from_axis_angle(&Vector3::x_axis(), self.pitch)
            * UnitQuaternion::from_axis_angle(&Vector3::z_axis(), self.roll);

        context.scene.graph[self.pivot]
            .local_transform_mut()
            .set_rotation(rotation);
    }

    fn restore_resources(&mut self, _resource_manager: ResourceManager) {
        // Restore resource handles here.
    }

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}

impl CameraController {
    pub fn handle_device_event(&mut self, device_event: &DeviceEvent) {
        if let DeviceEvent::MouseMotion { delta } = device_event {
            self.yaw -= (delta.0 as f32) * self.mouse_sensitivity * 0.001;
            self.pitch += (delta.1 as f32) * self.mouse_sensitivity * 0.001;
            self.roll += 0.0;
        }
    }
}

use fyrox::{
    core::{
        pool::Handle,
        reflect::prelude::*,
        uuid::{uuid, Uuid},
        visitor::prelude::*,
    },
    engine::resource_manager::ResourceManager,
    event::Event,
    impl_component_provider,
    scene::node::{Node, TypeUuidProvider},
    script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};

#[derive(Visit, Reflect, Default, Debug, Clone)]
pub struct Ball {
    pub body: Handle<Node>,
}

impl_component_provider!(Ball);

impl TypeUuidProvider for Ball {
    fn type_uuid() -> Uuid {
        uuid!("0c483d4c-7650-4bfb-8346-88bb18d06c97")
    }
}

impl ScriptTrait for Ball {
    fn on_init(&mut self, _context: &mut ScriptContext) {
        // Put initialization logic here.
    }

    fn on_start(&mut self, _context: &mut ScriptContext) {
        // let body = context.scene.graph[self.body].as_rigid_body_mut();
        // body.apply_force(Vector3::new(100.0, 0.0, 0.0))
    }

    fn on_deinit(&mut self, _context: &mut ScriptDeinitContext) {
        // Put de-initialization logic here.
    }

    fn on_os_event(&mut self, _event: &Event<()>, _context: &mut ScriptContext) {
        // Respond to OS events here.
    }

    fn on_update(&mut self, _context: &mut ScriptContext) {}

    fn restore_resources(&mut self, _resource_manager: ResourceManager) {
        // Restore resource handles here.
    }

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}

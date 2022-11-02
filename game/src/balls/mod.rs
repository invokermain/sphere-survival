use fyrox::{
    core::{
        algebra::Vector3,
        inspect::prelude::*,
        reflect::Reflect,
        uuid::{uuid, Uuid},
        visitor::prelude::*,
    },
    engine::resource_manager::ResourceManager,
    event::Event,
    impl_component_provider,
    scene::{node::TypeUuidProvider, Scene},
    script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};

use self::ball::Ball;

mod ball;

#[derive(Visit, Reflect, Inspect, Default, Debug, Clone)]
pub struct Balls {
    pub balls: Vec<Ball>,
}

impl Balls {
    pub fn new(scene: &mut Scene) -> Self {
        let ball_one = Ball::new(scene, Vector3::new(50.0, 0.0, 0.0));
        let ball_two = Ball::new(scene, Vector3::new(00.0, 50.0, 0.0));
        let ball_three = Ball::new(scene, Vector3::new(00.0, 0.0, 50.0));

        let balls = vec![ball_one, ball_two, ball_three];

        Self { balls }
    }
}
impl_component_provider!(Balls);

impl TypeUuidProvider for Balls {
    fn type_uuid() -> Uuid {
        uuid!("0c483d4c-7650-4bfb-8346-88bb18d06c97")
    }
}

impl ScriptTrait for Balls {
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
        // Put object logic here.
    }

    fn restore_resources(&mut self, resource_manager: ResourceManager) {
        // Restore resource handles here.
    }

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}

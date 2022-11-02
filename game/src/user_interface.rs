use fyrox::{
    core::{
        inspect::prelude::*,
        pool::Handle,
        reflect::Reflect,
        uuid::{uuid, Uuid},
        visitor::prelude::*,
    },
    engine::resource_manager::ResourceManager,
    event::Event,
    gui::{text::TextBuilder, widget::WidgetBuilder, UiNode, UserInterface},
    impl_component_provider,
    scene::node::TypeUuidProvider,
    script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};

#[derive(Visit, Reflect, Inspect, Default, Debug, Clone)]
pub struct GameUI {
    pub fps_counter: Handle<UiNode>,
}

impl GameUI {
    pub fn new(ui: &mut UserInterface) -> Self {
        let fps_counter = TextBuilder::new(WidgetBuilder::new())
            .with_text("0")
            .build(&mut ui.build_ctx());

        Self { fps_counter }
    }
}

impl_component_provider!(GameUI);

impl TypeUuidProvider for GameUI {
    fn type_uuid() -> Uuid {
        uuid!("d67f0d47-66a4-4e17-ac6a-9dcda85e5277")
    }
}

impl ScriptTrait for GameUI {
    fn on_init(&mut self, _context: &mut ScriptContext) {}

    fn on_start(&mut self, _context: &mut ScriptContext) {}

    fn on_deinit(&mut self, _context: &mut ScriptDeinitContext) {}

    fn on_os_event(&mut self, _event: &Event<()>, _context: &mut ScriptContext) {}

    fn on_update(&mut self, _context: &mut ScriptContext) {}

    fn restore_resources(&mut self, _resource_manager: ResourceManager) {}

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}

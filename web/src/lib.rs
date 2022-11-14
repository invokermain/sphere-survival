extern crate console_error_panic_hook;
use std::{panic, sync::Arc};

use fyrox::{
    core::pool::Handle,
    engine::{executor::Executor, SerializationContext},
    event::Event,
    event_loop::ControlFlow,
    gui::message::UiMessage,
    plugin::{Plugin, PluginConstructor, PluginContext},
    scene::{Scene, SceneLoader},
};
use mason::GameConstructor;
use wasm_bindgen::prelude::wasm_bindgen;

pub struct WebGameConstructor;

impl PluginConstructor for WebGameConstructor {
    fn create_instance(
        &self,
        _override_scene: Handle<Scene>,
        context: PluginContext,
    ) -> Box<dyn Plugin> {
        Box::new(WebGame::new(context))
    }
}

pub struct WebGame {}

async fn load_scene(context: Arc<SerializationContext>) {
    SceneLoader::from_file("../data/scene.rgs", context.clone())
        .await
        .expect("Problem loading scene!");
}

impl WebGame {
    pub fn new(context: PluginContext) -> Self {
        wasm_bindgen_futures::spawn_local(load_scene(context.serialization_context.clone()));

        Self {}
    }
}

impl Plugin for WebGame {
    fn on_deinit(&mut self, _context: PluginContext) {}

    fn update(&mut self, _context: &mut PluginContext, _control_flow: &mut ControlFlow) {}

    fn on_os_event(
        &mut self,
        _event: &Event<()>,
        _context: PluginContext,
        _control_flow: &mut ControlFlow,
    ) {
    }

    fn on_ui_message(
        &mut self,
        _context: &mut PluginContext,
        _message: &UiMessage,
        _control_flow: &mut ControlFlow,
    ) {
    }
}

#[wasm_bindgen]
pub fn run_game() {
    // this hook makes wasm panic messages much better
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let mut executor = Executor::new();
    executor.add_plugin_constructor(GameConstructor);
    executor.run()
}

//! Game project.
mod ball;
mod camera_controller;
mod player;
mod user_interface;

use ball::Ball;
use camera_controller::CameraController;
use fyrox::{
    core::{algebra::Vector3, pool::Handle},
    event::Event,
    event_loop::ControlFlow,
    gui::{
        message::{MessageDirection, UiMessage},
        text::TextMessage,
    },
    plugin::{Plugin, PluginConstructor, PluginContext, PluginRegistrationContext},
    scene::{loader::AsyncSceneLoader, Scene},
    utils::log::Log,
};

use player::Player;
use user_interface::GameUI;

pub struct GameConstructor;

impl PluginConstructor for GameConstructor {
    fn register(&self, context: PluginRegistrationContext) {
        context
            .serialization_context
            .script_constructors
            .add::<Ball>("Ball")
            .add::<CameraController>("CameraController")
            .add::<Player>("PlayerController");
    }

    fn create_instance(
        &self,
        override_scene: Handle<Scene>,
        context: PluginContext,
    ) -> Box<dyn Plugin> {
        Box::new(Game::new(override_scene, context))
    }
}

pub struct Game {
    scene: Handle<Scene>,
    loader: Option<AsyncSceneLoader>,
    ui: GameUI,
}

impl Game {
    pub fn new(override_scene: Handle<Scene>, context: PluginContext) -> Self {
        let mut loader = None;
        let scene = if override_scene.is_some() {
            override_scene
        } else {
            loader = Some(AsyncSceneLoader::begin_loading(
                "data/scene.rgs".into(),
                context.serialization_context.clone(),
                context.resource_manager.clone(),
            ));
            Default::default()
        };

        Self {
            scene,
            loader,
            ui: GameUI::new(context.user_interface),
        }
    }
}

impl Plugin for Game {
    fn on_deinit(&mut self, _context: PluginContext) {}

    fn update(&mut self, context: &mut PluginContext, _control_flow: &mut ControlFlow) {
        if let Some(loader) = self.loader.as_ref() {
            if let Some(result) = loader.fetch_result() {
                match result {
                    Ok(scene) => {
                        self.scene = context.scenes.add(scene);
                        context.scenes[self.scene].graph.physics.gravity = Vector3::zeros();
                    }
                    Err(err) => Log::err(err),
                }
            }
        }

        // update UI
        let fps = format!("{:.1} fps", 1.0 / context.dt);
        context.user_interface.send_message(TextMessage::text(
            self.ui.fps_counter,
            MessageDirection::ToWidget,
            fps,
        ))
    }

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

//! Game project.
mod balls;
mod player;
mod user_interface;

use balls::Balls;
use fyrox::{
    core::{futures::executor::block_on, pool::Handle},
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    gui::{
        message::{MessageDirection, UiMessage},
        text::TextMessage,
    },
    plugin::{Plugin, PluginConstructor, PluginContext, PluginRegistrationContext},
    scene::{Scene, SceneLoader},
};
use player::Player;
use user_interface::GameUI;

pub struct GameConstructor;

impl PluginConstructor for GameConstructor {
    fn register(&self, _context: PluginRegistrationContext) {}

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
    player: Player,
    balls: Balls,
    ui: GameUI,
}

impl Game {
    pub fn new(override_scene: Handle<Scene>, context: PluginContext) -> Self {
        let scene_handle = if override_scene.is_some() {
            override_scene
        } else {
            // Load a scene from file if there is no override scene specified.
            let scene = block_on(
                block_on(SceneLoader::from_file(
                    "data/scene.rgs",
                    context.serialization_context.clone(),
                ))
                .unwrap()
                .finish(context.resource_manager.clone()),
            );

            context.scenes.add(scene)
        };

        let scene = context.scenes.try_get_mut(scene_handle).unwrap();

        let player = Player::new(scene);

        let balls = Balls::new(scene);

        Self {
            player,
            balls,
            scene: scene_handle,
            ui: GameUI::new(context.user_interface),
        }
    }
}

impl Plugin for Game {
    fn on_deinit(&mut self, _context: PluginContext) {}

    fn update(&mut self, context: &mut PluginContext, _control_flow: &mut ControlFlow) {
        let scene = &mut context.scenes[self.scene];

        // for ball in &self.balls.balls {}

        self.player.update(scene, context.dt);

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
        event: &Event<()>,
        _context: PluginContext,
        _control_flow: &mut ControlFlow,
    ) {
        match event {
            Event::DeviceEvent { event, .. } => {
                self.player.handle_device_event(event);
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                self.player.handle_key_event(input);
            }
            _ => (),
        }
    }

    fn on_ui_message(
        &mut self,
        _context: &mut PluginContext,
        _message: &UiMessage,
        _control_flow: &mut ControlFlow,
    ) {
    }
}

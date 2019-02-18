use amethyst::prelude::*;
use amethyst::renderer::VirtualKeyCode;
use amethyst::input::is_key_down;

pub struct PausedState;

impl EmptyState for PausedState {

    fn on_start(&mut self, _data: StateData<()>) {
        println!("Paused state initiated");
    }

    fn handle_event(&mut self, _data: StateData<()>, event: StateEvent) -> EmptyTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Space) {
                return Trans::Pop;
            }
        }
        Trans::None
    }
}
use std::sync::{Mutex, Arc};

use events::DuplexEventsPlugin;

pub mod events;
pub mod frontend;
pub mod game;

pub struct SharedState {
    pub name: String,
}

pub type Shared<T> = Arc<Mutex<T>>;

fn main() {
    let shared = Arc::new(Mutex::new(SharedState { name: "This can be used for shared state".to_string()} ));
    let ((tx_events, rx_events), duplex_events_plugin) = DuplexEventsPlugin::create();
    frontend::yew_main(tx_events, rx_events, shared.clone());
    game::bevy_main(duplex_events_plugin, shared);
}
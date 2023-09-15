use yew::prelude::*;

use crate::{events::{TxInputEvent, CounterEvent, RxOutputEvent}, SharedState, Shared};

pub fn yew_main(tx_events: TxInputEvent, rx_events: RxOutputEvent, shared: Shared<SharedState>) {
    let document = gloo::utils::document();
    let root = document.query_selector("#yew").unwrap().unwrap();
    yew::Renderer::<App>::with_root_and_props(root, Props { tx_events, rx_events, shared }).render();
}

#[derive(Properties)]
pub struct Props {
    pub shared: Shared<SharedState>,
    pub tx_events: TxInputEvent,
    pub rx_events: RxOutputEvent,
}

impl PartialEq for Props {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

#[function_component(App)]
pub fn app(props: &Props) -> Html {
    let counter = use_state(|| 0);
    let tx_events = props.tx_events.clone();
    let name = props.shared.lock().unwrap().name.clone();
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
            tx_events.send( crate::events::InputEvent::Counter(CounterEvent { value })).expect("could not send event");
        }
    };

    html! {
        <>
            <div>
                <p>{ name }</p>
                <button {onclick}>{ "+1" }</button>
                <p>{ *counter }</p>
            </div>
        </>
    }
}

use yew::prelude::*;

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        log::info!("start view");
        let mut class = Classes::new();
        for i in 0..100 {
            class.push(format!("test-class-{i}"));
        }
        log::info!("end view");

        html! {"If you reload this page, wekbit crashes sometimes."}
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

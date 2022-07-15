use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let canvas_ref = NodeRef::default();

    html! {
        <canvas
            ref={canvas_ref}
            width={1280}
            height={720}
        />
    }
}

fn main() {
    yew::start_app::<App>();
}

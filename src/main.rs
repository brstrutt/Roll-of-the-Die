use yew::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use wasm_bindgen::{JsCast, JsValue};

#[function_component(App)]
fn app() -> Html {
    let ui_ref = NodeRef::default();
    let width = 1280;
    let height = 720;
    
    let on_click = {
        let cur_input = ui_ref.clone();
        Callback::from( move |_e: MouseEvent| {
            draw_smiley_face_on_canvas(&cur_input);
        })
    };

    html! { 
        <>
            <canvas
                ref={ui_ref}
                width={width.to_string()}
                height={height.to_string()}
            />
            <button onclick={on_click}>{"Generate Noise Image"}</button>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}

fn get_canvas_context(canvas_reference: &NodeRef) -> Option<CanvasRenderingContext2d> {
    let canvas = canvas_reference.cast::<HtmlCanvasElement>()?;

    let canvas_context_object = canvas.get_context("2d");
    if canvas_context_object.is_err() { return None; }

    let canvas_context = canvas_context_object.unwrap()?.dyn_into::<CanvasRenderingContext2d>();
    if canvas_context.is_err() { return None; }

    Some(canvas_context.unwrap())
}

fn draw_smiley_face_on_canvas(canvas: &NodeRef) {
    let canvas_context = get_canvas_context(&canvas);
    if canvas_context.is_some() {
        let context = canvas_context.unwrap();
        context.begin_path();

        // Draw the outer circle.
        context
            .arc(75.0, 75.0, 50.0, 0.0, std::f64::consts::PI * 2.0)
            .unwrap();

        // Draw the mouth.
        context.move_to(110.0, 75.0);
        context.arc(75.0, 75.0, 35.0, 0.0, std::f64::consts::PI).unwrap();

        // Draw the left eye.
        context.move_to(65.0, 65.0);
        context
            .arc(60.0, 65.0, 5.0, 0.0, std::f64::consts::PI * 2.0)
            .unwrap();

        // Draw the right eye.
        context.move_to(95.0, 65.0);
        context
            .arc(90.0, 65.0, 5.0, 0.0, std::f64::consts::PI * 2.0)
            .unwrap();

        context.stroke();
    }
}
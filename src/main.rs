use yew::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use wasm_bindgen::{JsCast};

#[function_component(App)]
fn app() -> Html {
    let ui_ref = NodeRef::default();
    let width = 1280;
    let height = 720;

    let happy = use_state(|| false);
    
    let on_click = {
        let cur_input = ui_ref.clone();
        let happy = happy.clone();
        Callback::from( move |_e: MouseEvent| {

            let context = get_canvas_context(&cur_input);

            if context.is_none() { return; }
            let context = context.unwrap();

            clear_canvas(&context);
            if *happy {
                draw_smiley_face_on_canvas(&context);
            }
            else {
                draw_sad_face_on_canvas(&context);
            }

            happy.set(!*happy);
        })
    };

    html! { 
        <>
            <canvas
                ref={ui_ref}
                width={width.to_string()}
                height={height.to_string()}
            />
            <button onclick={on_click}>{"Toggle Face"}</button>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::debug!("App is starting");
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

fn clear_canvas(context: &CanvasRenderingContext2d) {
    context.clear_rect(0.0, 0.0, 1280.0, 720.0);
}

fn draw_smiley_face_on_canvas(context: &CanvasRenderingContext2d) {
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

fn draw_sad_face_on_canvas(context: &CanvasRenderingContext2d) {
    context.begin_path();

    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, std::f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, std::f64::consts::PI, 0.0).unwrap();

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
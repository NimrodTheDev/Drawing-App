
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;

use crate::art_engine::get_canvas_ctx;
use crate::layout::art_layout::{get_layout_context, ArtAction};
use crate::layout::form_context::{use_form_context, FormContextStoreFields};
use crate::components::{
    text_form::TypographyForm,
    shape_form::ShapeForm,
};
#[component]
pub fn DrawView() -> impl IntoView {
    let ctx = get_layout_context();
    let form_ctx = use_form_context();
    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();
    let input_position: RwSignal<Option<(f64, f64, f64, f64)>> = RwSignal::new(None);
    let tab_control = RwSignal::new(false);
    let text_value = RwSignal::new(String::new());
    Effect::new(move || {
        if let Some(canvas) = canvas_ref.get() {
            let rect = canvas.get_bounding_client_rect();

            // make buffer match CSS display size
            canvas.set_width(rect.width() as u32);
            canvas.set_height(rect.height() as u32);

            let original_ctx = get_canvas_ctx(canvas.into());
            ctx.set_ctx.set(Some(original_ctx));
        }
    });

    view! {
        <>
        <div class="relative bg-pink w-full h-full flex items-start justify-start mx-auto">
          <canvas
            on:click:target=move|e|{
                let canvas = canvas_ref.get();
                        if let Some(canvas) = canvas {
                            let rect = canvas.get_bounding_client_rect();
                            // position relative to canvas, not whatever element was clicked
                            let screen_x = e.client_x() as f64 - rect.left();
                            let screen_y = e.client_y() as f64 - rect.top();

                            let (pan_x, pan_y) = ctx.pan.get();
                            let zoom = ctx.zoom.get();

                            let world_x = (screen_x - pan_x as f64) / zoom;
                            let world_y = (screen_y - pan_y as f64) / zoom;


                            match ctx.is_active.get().as_str() {
                              "Text" => {
                                    input_position.set(Some((world_x, world_y, e.client_x().into(), e.client_y().into())));
                                }
                              "Shapes"=>{
                                    input_position.set(Some((
                                        world_x,
                                        world_y,
                                        e.client_x().into(),
                                        e.client_y().into(),
                                    )));
                                }
                                _=>{

                                }
                            };
                };

            }
            node_ref=canvas_ref
            id="canvas"
            class="w-full h-[calc(100%-30px)] rounded-3xl bg-white backdrop-blur-xl shadow-2xl shadow-indigo-500/30 sticky top-[56px]"
          ></canvas>
        // Inside your main, after the canvas

        {move || {
    input_position.get().map(|at| {
        match ctx.is_active.get().as_str() {
            "Text" => {
                view! {
                    <div
                        style=move || format!(
                            "position: fixed;
                             left: {}px;
                             top: {}px;
                             z-index: 20;
                             display: flex;
                             flex-direction: column;
                             gap: 4px;
                             transform: translateY(calc(-100% + 40px));",
                            at.2,
                            at.3
                        )
                    >
                // the actual text input
                <input
                    contenteditable="true"
                    autofocus=true
                    style=move || format!(
                        "min-width: 80px; min-height: 24px; font-size: {}px; color: black; \
                         background: rgba(0,0,0,0.05); border: 1px dashed #aaa; \
                         padding: 2px 6px; outline: none; white-space: pre;",
                        form_ctx.TextForm().get().font_size
                    )
                    on:input:target=move|e|{

                        let el = e.target();
                        let text = el.value().clone();
                        text_value.set(text.clone());
                    }
                    on:keydown:target=move |e| {
                        let ke = e.clone();
                        // let el = e.target();
                        let text = text_value.get().clone();
                        // text_value.set(text.clone());
                        if ke.key() == "Enter" && !ke.shift_key() {
                            if !text.clone().trim().is_empty() {
                                let value =  form_ctx.TextForm().get();
                                ctx.set_objects.update(|objs| {
                                    objs.push(ArtAction::text {
                                    x: at.0,
                                    y: at.1,
                                    text: text.trim().to_string(),
                                    text_align: value.text_align,
                                    font_family: value.font_family,
                                    font_size: value.font_size,
                                    font_weight: value.font_weight,
                                    italic: value.italic,
                                    underline: value.underline,
                                    letter_spacing: value.letter_spacing,
                                    fill_color: value.fill_color,
                                    stroke_text: value.stroke_text,
                                    stroke_color: value.stroke_color
                                });
                                text_value.set("".into());
                                });
                            }
                            input_position.set(None);
                        }
                        if ke.key() == "Escape" {
                            input_position.set(None);
                        }
                    }
                />

                // toolbar row at the bottom
                <div style="display: flex; justify-content: space-between; align-items: center; padding: 2px 4px; background: #151821; border-radius: 4px;">

                    // confirm/tick
                    <button
                        style="background: none; border: none; color: #ec4899; cursor: pointer; font-size: 14px; padding: 2px 4px; display: flex; align-items: center;"
                        on:click=move |_| {
                            // find the contenteditable above and commit
                            // simplest: just close, blur will have committed
                            let text = text_value.get();
                            let value =  form_ctx.TextForm().get();
                                ctx.set_objects.update(|objs| {
                                    objs.push(ArtAction::text {
                                        x: at.0,
                                        y: at.1,
                                        text: text.trim().to_string(),
                                        text_align: value.text_align,
                                        font_family: value.font_family,
                                        font_size: value.font_size,
                                        font_weight: value.font_weight,
                                        italic: value.italic,
                                        underline: value.underline,
                                        letter_spacing: value.letter_spacing,
                                        fill_color: value.fill_color,
                                        stroke_text: value.stroke_text,
                                        stroke_color: value.stroke_color
                                    });
                                });
                                text_value.set("".into());
                            input_position.set(None);
                        }
                    >
                        <i class="bi bi-check-lg"></i>
                    </button>

                    // move handle — draggable
                    // <div
                    //     style="color: #888; cursor: grab; font-size: 14px; padding: 2px 4px; display: flex; align-items: center; user-select: none;"
                    //     // you can wire mousedown here later for drag-to-reposition
                    // >
                    //     <i class="bi bi-grip-horizontal"></i>
                    // </div>

                    // dismiss
                    <button
                        style="background: none; border: none; color: #888; cursor: pointer; font-size: 14px; padding: 2px 4px; display: flex; align-items: center;"
                        on:click=move |_| input_position.set(None)
                    >
                        <i class="bi bi-x"></i>
                    </button>

                </div>
            </div>
                }.into_any()
            }

            "Shapes" => {
                view! {
                    <div
                        style=move || format!(
                            "position: fixed;
                             left: {}px;
                             top: {}px;
                             z-index: 20;
                             display: flex;
                             flex-direction: column;
                             gap: 4px;
                             ",
                            at.2,
                            at.3
                        )
                    >
                        <div
                            style=move || {
                                let shape = form_ctx.ShapeForm().get();

                                format!(
                                    "
                                    width:{}px;
                                    height:{}px;
                                    background:{};
                                    border:1px solid {};
                                    ",
                                    shape.width,
                                    shape.height,
                                    shape.fill_color,
                                    shape.stroke_color,
                                )
                            }
                        />

                        <div
                            style="
                                display:flex;
                                justify-content:space-between;
                                align-items:center;
                                padding:2px 4px;
                                background:#151821;
                                border-radius:4px;
                            "
                        >
                            <button
                                on:click=move |_| {
                                    let settings = form_ctx.ShapeForm().get();

                                    ctx.set_objects.update(|objs| {
                                        objs.push(
                                            ArtAction::rect {
                                                x: at.0,
                                                y: at.1,
                                                width: settings.width,
                                                height: settings.height,
                                                settings
                                            }
                                        );
                                    });

                                    input_position.set(None);
                                }
                            >
                                <i class="bi bi-check-lg"></i>
                            </button>

                            <button
                                on:click=move |_| {
                                    input_position.set(None);
                                }
                            >
                                <i class="bi bi-x"></i>
                            </button>
                        </div>
                    </div>
                }.into_any()
            }

            _ => view! {}.into_any(),
        }
    })
}}


        </div>
        <div class="w-[30px] h-[500px] flex sticky top-[56px] bg-aux ">
            <div class="w-full h-full flex items-center justify-center " on:click:target=move|e|{
                e.stop_propagation();
                tab_control.update(|e|{
                   *e = !e.clone();
                });
            }>
                <i class="bi bi-door-open"></i>
                <div class=move||format!("w-[200px] h-full absolute bg-aux top-0 right-[110%] z-10 overflow-scroll {}", if tab_control.get() {"hidden"} else {""}) on:click=move |e| e.stop_propagation()>
                    <Tab/>
                </div>
            </div>
        </div>
        </>
    }
}


#[component]
pub fn tab()-> impl IntoView{
    let ctx = get_layout_context();

    view! {
        {
            move|| match ctx.is_active.get().as_str() {
            "Text" => {
                Some(
                    view! {
                        <TypographyForm/>
                        }.into_any()
                    )
                }
            "Shapes" => {
                Some(
                    view! {
                        <ShapeForm/>
                        }.into_any()
                    )
                }
                _=>{
                    None
                }
            }
        }
    }
}


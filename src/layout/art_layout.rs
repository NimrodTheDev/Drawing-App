use crate::art_engine::{get_canvas_ctx, render, zoom_and_pan};
use leptos::ev::click;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos::reactive::signal;
use leptos::server_fn::serde::Serialize;
use leptos::wasm_bindgen::{JsCast, JsValue};
use leptos::web_sys::MouseEvent;
use std::rc::Rc;
use std::string::ToString;
use std::sync::Arc;
use std::time::Duration;
use web_sys::CanvasRenderingContext2d;

#[derive(Clone)]
pub struct Tab {
    pub title: String,
    pub icon: String,
    pub click_event: Callback<leptos::ev::MouseEvent>,
}

use crate::layout::form_context::{FormContext, FormProvider, ShapeSettings};
use serde::Deserialize;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "PascalCase")]
#[derive(Clone)]
pub enum ArtAction {
    text {
        x: f64,
        y: f64,
        text: String,
        font_size: f64,
        font_weight: String,
        font_family: String,
        italic: bool,
        underline: bool,
        letter_spacing: f64,
        text_align: String,
        fill_color: String,
        stroke_text: bool,
        stroke_color: String,
    },
    rect {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        settings: ShapeSettings,
    },
}
impl From<ArtAction> for JsValue {
    fn from(art_action: ArtAction) -> JsValue {
        use leptos::web_sys::js_sys::{Object, Reflect};
        let objects = Object::new();
        match art_action {
            ArtAction::text {
                x,
                y,
                text,
                font_size,
                font_weight,
                font_family,
                italic,
                underline,
                letter_spacing,
                text_align,
                fill_color,
                stroke_text,
                stroke_color,
            } => {
                Reflect::set(&objects, &"type".into(), &"text".into()).unwrap();
                Reflect::set(&objects, &"text".into(), &text.into()).unwrap();
                Reflect::set(&objects, &"x".into(), &x.into()).unwrap();
                Reflect::set(&objects, &"y".into(), &y.into()).unwrap();

                Reflect::set(&objects, &"font_size".into(), &font_size.into()).unwrap();
                Reflect::set(&objects, &"font_weight".into(), &font_weight.into()).unwrap();
                Reflect::set(&objects, &"font_family".into(), &font_family.into()).unwrap();

                Reflect::set(&objects, &"italic".into(), &italic.into()).unwrap();
                Reflect::set(&objects, &"underline".into(), &underline.into()).unwrap();

                Reflect::set(&objects, &"letter_spacing".into(), &letter_spacing.into()).unwrap();

                Reflect::set(&objects, &"text_align".into(), &text_align.into()).unwrap();

                Reflect::set(&objects, &"fill_color".into(), &fill_color.into()).unwrap();
                Reflect::set(&objects, &"stroke_text".into(), &stroke_text.into()).unwrap();
                Reflect::set(&objects, &"stroke_color".into(), &stroke_color.into()).unwrap();
            }
            ArtAction::rect {
                x,
                y,
                width,
                height,
                settings
            } => {
                Reflect::set(&objects, &"type".into(), &"rect".into()).unwrap();
                Reflect::set(&objects, &"x".into(), &x.into()).unwrap();
                Reflect::set(&objects, &"y".into(), &y.into()).unwrap();
                Reflect::set(&objects, &"width".into(), &width.into()).unwrap();
                Reflect::set(&objects, &"height".into(), &height.into()).unwrap();
                Reflect::set(&objects, &"settings".into(), &serde_wasm_bindgen::to_value(&settings).unwrap()).unwrap();
            }
        };
        objects.into()
    }
}

#[derive(Copy, Clone)]
pub struct ArtLayoutCtx {
    pub ctx: ReadSignal<Option<web_sys::CanvasRenderingContext2d>>,
    pub set_ctx: WriteSignal<Option<web_sys::CanvasRenderingContext2d>>,
    pub objects: ReadSignal<Vec<ArtAction>>,
    pub set_objects: WriteSignal<Vec<ArtAction>>,
    pub is_active: ReadSignal<String>,
    pub pan: ReadSignal<(i32, i32)>,
    pub zoom: ReadSignal<f64>,
}
#[component]
pub fn ArtLayout(children: Children) -> impl IntoView {
    let (is_active, set_is_active) = signal("Select".to_string());
    let click_select = Callback::new(move |_| {
        set_is_active.set("Select".into());
    });
    let click_brush = Callback::new(move |_| {
        set_is_active.set("Brush".into());
    });
    let click_eraser = Callback::new(move |_| {
        set_is_active.set("Eraser".into());
    });
    let click_shapes = Callback::new(move |_| {
        set_is_active.set("Shapes".into());
    });
    let click_text = Callback::new(move |_| {
        set_is_active.set("Text".into());
    });
    let click_line = Callback::new(move |_| {
        set_is_active.set("Line".into());
    });
    let click_fill = Callback::new(move |_| {
        set_is_active.set("Fill".into());
    });
    let click_eyedropper = Callback::new(move |_| {
        set_is_active.set("Eyedropper".into());
    });
    let click_hand = Callback::new(move |_| {
        set_is_active.set("Hand".into());
    });

    let menu: Vec<Tab> = vec![
        Tab {
            title: "Select".to_string(),
            icon: "<i class='bi bi-cursor-fill'></i>".to_string(),
            click_event: click_select.clone(),
        },
        Tab {
            title: "Brush".to_string(),
            icon: "<i class='bi bi-brush'></i>".to_string(),
            click_event: click_brush.clone(),
        },
        Tab {
            title: "Eraser".to_string(),
            icon: "<i class='bi bi-eraser'></i>".to_string(),
            click_event: click_eraser.clone(),
        },
        Tab {
            title: "Shapes".to_string(),
            icon: "<i class='bi bi-bounding-box'></i>".to_string(),
            click_event: click_shapes.clone(),
        },
        Tab {
            title: "Text".to_string(),
            icon: "<i class='bi bi-type'></i>".to_string(),
            click_event: click_text.clone(),
        },
        Tab {
            title: "Line".to_string(),
            icon: "<i class='bi bi-slash-lg'></i>".to_string(),
            click_event: click_line.clone(),
        },
        Tab {
            title: "Fill".to_string(),
            icon: "<i class='bi bi-paint-bucket'></i>".to_string(),
            click_event: click_fill.clone(),
        },
        Tab {
            title: "Eyedropper".to_string(),
            icon: "<i class='bi bi-eyedropper'></i>".to_string(),
            click_event: click_eyedropper.clone(),
        },
        Tab {
            title: "Hand".to_string(),
            icon: "<i class='bi bi-hand-index-thumb'></i>".to_string(),
            click_event: click_hand.clone(),
        },
    ];

    let is_active_func = move |arg: &str| -> bool { is_active.get() == arg.to_string() };
    let (ctx, set_ctx) = signal::<Option<CanvasRenderingContext2d>>(None);
    let (pan, set_pan) = signal((0, 0));
    let (zoom, set_zoom) = signal::<f64>(1.into());
    let (is_panning, set_is_panning) = signal(false);
    let (start_pan_xy, set_start_pan_xy) = signal((0, 0));

    let (objects, set_objects) = signal::<Vec<ArtAction>>(vec![]);

    Effect::new(move || {
        if let Some(ctx) = ctx.get() {
            let window = leptos::web_sys::window().expect("should have a window");
            let doc = window.document().expect("should have a document");
            let canvas = doc
                .get_element_by_id("canvas")
                .expect("element not found")
                .dyn_into::<leptos::web_sys::HtmlCanvasElement>()
                .expect("should be a canvas element");
            render(
                ctx,
                &canvas,
                objects
                    .get()
                    .into_iter()
                    .map(|f| f.clone().into())
                    .collect(),
                pan.get().0.into(),
                pan.get().1.into(),
                zoom.get().into(),
            );
        }
    });

    FormProvider();
    provide_context(ArtLayoutCtx {
        ctx,
        set_ctx,
        objects,
        set_objects,
        is_active,
        pan,
        zoom,
    });
    let on_mousedown = move |e: MouseEvent| {
        set_is_panning.set(true);
        let mousex = e.offset_x();
        let mousey = e.offset_y();
        let x = mousex - pan.get().0;
        let y = mousey - pan.get().1;
        set_start_pan_xy.set((x, y));
    };

    let on_mousemove = move |e: MouseEvent| {
        if is_panning.get() && is_active.get().as_str() == "Hand" {
            let x = e.offset_x();
            let y = e.offset_y();
            let newx = x - start_pan_xy.get().0;
            let newy = y - start_pan_xy.get().1;
            set_pan.set((newx, newy));
        };
    };

    let on_mouseup = move |_e| {
        set_is_panning.set(false);
    };

    let on_zoom_in = move |_e| {
        set_zoom.update(|z| *z = (*z + 0.1).clamp(0.1, 10.0));
    };

    let on_zoom_out = move |_e| {
        set_zoom.update(|z| *z = (*z - 0.1).clamp(0.1, 10.0));
    };

    let style = move || {
        let cursor = match is_active.get().as_str() {
            // "Text" => {
            //     "url(\"data:image/svg+xml,%3Csvg xmlns=%22http://www.w3.org/2000/svg%22 width=%2232%22 height=%2232%22%3E%3Ctext x=%222%22 y=%2222%22 font-size=%2220%22 font-family=%22Arial%22 fill=%22black%22%3EAa%3C/text%3E%3C/svg%3E\") 0 20, auto"
            // }
            "Hand" => {
                if is_panning.get() {
                    "grabbing"
                } else {
                    "grab"
                }
            }

            "Brush" => "crosshair",
            "Eraser" => "cell",
            "Zoom" => "zoom-in",
            "Eyedropper" => "copy",
            _ => "default",
        };
        format!("cursor: {cursor}")
    };

    view! {
        <main
            style=move || {
                let cursor = match is_active.get().as_str() {
                    // "Text" => {
                    //     "url(\"data:image/svg+xml,%3Csvg xmlns=%22http://www.w3.org/2000/svg%22 width=%2232%22 height=%2232%22%3E%3Ctext x=%222%22 y=%2222%22 font-size=%2220%22 font-family=%22Arial%22 fill=%22black%22%3EAa%3C/text%3E%3C/svg%3E\") 0 20, auto"
                    // }

                    "Hand" => {
                        if is_panning.get() {
                            "grabbing"
                        } else {
                            "grab"
                        }
                    }

                    "Brush" => "crosshair",
                    "Eraser" => "cell",
                    "Zoom" => "zoom-in",
                    "Eyedropper" => "copy",
                    _ => "default",
                };
                format!("cursor: {cursor}")
            }
            class=(format!("h-full w-full box-border p-5 dark:bg-zinc-950 bg-zinc-50 flex gap-4 {}", if is_panning.get() {
             "cursor-pinch"
            }else {
                ""
            }))
                on:mousedown=on_mousedown
                on:mousemove=on_mousemove
                on:mouseup=on_mouseup
        >
        <div class="bg-aux flex flex-col gap-6 rounded w-fit h-fit p-4 sticky top-[56px]">
            {
                move|| menu.clone().into_iter().map(move|tab|{
                    let click_ev = tab.click_event.clone();
                    let on_click_tab = move |e| {
                        click_ev.run(e);
                    };
                    view! {
                        <div on:click=on_click_tab
                        class=format!("flex gap-3 text-sm items-center justify-center cursor-pointer {}", if is_active_func(tab.title.clone().as_str()) {"text-button"} else {""})
                        >
                            <span class="text-[20px]"><div inner_html=tab.icon.clone()></div></span>
                            {
                            let title = tab.title.clone();
                                move || {
                                    if is_active_func(title.as_str()) {
                                        Some(title.clone())
                                    } else {
                                        None
                                    }
                                }
                            }
                        </div>
                    }
                }).collect_view()
            }
        </div>
        {children()}

        <div class="fixed bottom-[20px] flex gap-2 z-10 items-center p-1 bg-aux rounded-sm">
            <span class="px-2 text-button cursor-pointer" on:click=on_zoom_in>+</span>
            <span class="text-sm">{move || format!("{}%", (zoom.get() * 100.0).round() as i32)}</span>
            <span class="px-2 text-button cursor-pointer" on:click=on_zoom_out>-</span>
        </div>
        </main>
    }
}

pub fn get_layout_context() -> ArtLayoutCtx {
    let ctx = expect_context::<ArtLayoutCtx>();
    ctx
}

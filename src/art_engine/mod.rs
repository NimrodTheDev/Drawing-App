use leptos::html::object;
use leptos::leptos_dom::logging::console_log;
use leptos::wasm_bindgen;
use leptos::wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use web_sys::js_sys::Reflect;
use crate::layout::art_layout::ArtAction;
use crate::wasm_bindgen::prelude::wasm_bindgen;
use serde_wasm_bindgen::from_value;

#[wasm_bindgen]
pub fn get_canvas_ctx(canvas:HtmlCanvasElement)-> CanvasRenderingContext2d {
    let window = leptos::web_sys::window().expect("should have a window");
    let doc = window.document().expect("should have a document");

    let js: String = "words".into();
    let res = canvas.style().set_property("border", "solid").ok();
    let ctx = canvas.get_context("2d").expect("A context").unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().expect("should be a canvas context 2d");
    ctx
}


#[wasm_bindgen]
pub fn zoom_and_pan(ctx: CanvasRenderingContext2d, scaleX: f64, skewX: f64, skewY: f64, scaleY: f64, panX: f64, panY: f64) {
    ctx.set_transform( scaleX, skewX, skewY, scaleY, panX, panY).ok();
}

#[wasm_bindgen]
pub fn render(ctx: CanvasRenderingContext2d, canvas: &HtmlCanvasElement, object: Vec<JsValue>, pan_x: f64, pan_y: f64, zoom: f64) {
    console_log(format!("rerendered {:?}", object).as_str());
    ctx.reset_transform().ok();
    ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    let _ = ctx.translate(pan_x, pan_y);
    let _ = ctx.scale(zoom, zoom);
    for obj in object {
        let obj_type = from_value::<ArtAction>(obj);
        match obj_type {
            Ok(ArtAction::text { x, y, text, font_size, font_weight, font_family, italic, underline, letter_spacing, text_align , fill_color, stroke_color, stroke_text}) => {
                // build font string — same format as CSS font property
                let style = if italic { "italic" } else { "normal" };
                let font = format!("{style} {font_weight} {font_size}px {font_family}");

                ctx.set_font(&font);

                ctx.set_text_align(text_align.as_str());

                // letter spacing (canvas API — supported in modern browsers)
                ctx.set_fill_style_str(fill_color.as_str());
                ctx.set_stroke_style_str(&stroke_color);
                ctx.set_text_baseline("Top");


                let _ = ctx.fill_text(&text, x, y).ok();
                if stroke_text {
                    ctx.stroke_text(&text, x, y).ok();
                }else{
                }
            }
            Ok(ArtAction::rect { x, y, width, height, settings }) => {
                let s = settings;

                ctx.set_global_alpha(s.fill_opacity);

                // shadow
                ctx.set_shadow_color(&s.shadow_color);
                ctx.set_shadow_blur(s.shadow_blur);
                ctx.set_shadow_offset_x(s.shadow_offset_x);
                ctx.set_shadow_offset_y(s.shadow_offset_y);

                // compositing
                ctx.set_global_composite_operation(&s.composite_operation).ok();

                // stroke
                ctx.set_stroke_style_str(&s.stroke_color);
                ctx.set_line_width(s.stroke_width);
                ctx.set_line_dash(&serde_wasm_bindgen::to_value(&s.stroke_dash).unwrap()).ok();

                // fill
                ctx.set_fill_style_str(&s.fill_color);

                // rotation — done via transform around shape center
                ctx.save();
                if s.rotation > 0f64 {
                    let cx = x + s.width / 2.0;
                    let cy = y + s.height / 2.0;

                    ctx.translate(cx, cy).ok();

                    ctx.rotate(s.rotation * std::f64::consts::PI / 180.0).ok();
                    ctx.translate(-cx, -cy).ok();
                }

                match s.shape.as_str() {
                    "rect" => {
                        ctx.fill_rect(
                            x,
                            y,
                            s.width,
                            s.height,
                        );

                        if s.stroke_width > 0.0 {
                            ctx.stroke_rect(
                                x,
                                y,
                                s.width,
                                s.height,
                            );
                        }
                    }

                    "circle" => {
                        ctx.begin_path();

                        let radius = s.width.min(s.height) / 2.0;

                        let _ = ctx.arc(
                            x + radius,
                            y + radius,
                            radius,
                            0.0,
                            std::f64::consts::PI * 2.0,
                        );

                        ctx.fill();

                        if s.stroke_width > 0.0 {
                            ctx.stroke();
                        }
                    }

                    "ellipse" => {
                        ctx.begin_path();

                        let _ = ctx.ellipse(
                            x + s.width / 2.0,
                            y + s.height / 2.0,
                            s.width / 2.0,
                            s.height / 2.0,
                            0.0,
                            0.0,
                            std::f64::consts::PI * 2.0,
                        );

                        ctx.fill();

                        if s.stroke_width > 0.0 {
                            ctx.stroke();
                        }
                    }

                    "triangle" => {
                        ctx.begin_path();

                        ctx.move_to(x + s.width / 2.0, y);
                        ctx.line_to(x + s.width, y + s.height);
                        ctx.line_to(x, y + s.height);

                        ctx.close_path();

                        ctx.fill();

                        if s.stroke_width > 0.0 {
                            ctx.stroke();
                        }
                    }

                    "diamond" => {
                        ctx.begin_path();

                        ctx.move_to(x + s.width / 2.0, y);
                        ctx.line_to(x + s.width, y + s.height / 2.0);
                        ctx.line_to(x + s.width / 2.0, y + s.height);
                        ctx.line_to(x, y + s.height / 2.0);

                        ctx.close_path();

                        ctx.fill();

                        if s.stroke_width > 0.0 {
                            ctx.stroke();
                        }
                    }

                    _ => {}
                }
            }
            Err(e)=>{
                console_log(format!("{e}").as_str());
            }
        }

    };
    ctx.restore();

}


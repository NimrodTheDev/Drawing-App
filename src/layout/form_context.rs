use reactive_stores::Store;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct TypographySettings {
    pub font_family: String,
    pub font_weight: String,
    pub font_size: f64,
    pub line_height: f64,
    pub letter_spacing: f64,
    pub text_align: String,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub stroke_text: bool,
    pub fill_color: String,
    pub stroke_color: String,

}

impl Default for TypographySettings {
    fn default() -> Self {
        Self {
            font_family: "sans".into(),
            font_weight: "400".into(),
            font_size: 16.0,
            line_height: 1.5,
            letter_spacing: 0.0,
            text_align: "left".into(),
            italic: false,
            underline: false,
            strikethrough: false,
            fill_color: "#000000".into(),
            stroke_text: false,
            stroke_color: "#000000".into(),
        }
    }
}



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ShapeSettings {

    pub shape: String,
    // geometry
    pub width: f64,
    pub height: f64,
    pub rotation: f64,        // degrees

    // fill
    pub fill_color: String,   // hex/rgba
    pub fill_opacity: f64,    // 0.0 - 1.0

    // stroke (border)
    pub stroke_color: String,
    pub stroke_width: f64,
    pub stroke_opacity: f64,
    pub stroke_dash: Vec<f64>, // e.g. vec![5.0, 3.0] for dashed, vec![] for solid
    pub stroke_style: String,

    // corners (for rect)
    pub border_radius: f64,

    // shadow
    pub shadow_color: String,
    pub shadow_blur: f64,
    pub shadow_offset_x: f64,
    pub shadow_offset_y: f64,

    // compositing
    pub global_alpha: f64,          // 0.0 - 1.0, overall opacity
    pub composite_operation: String, // "source-over", "multiply", "screen" etc
}

impl Default for ShapeSettings {
    fn default() -> Self {
        Self {
            shape: "rect".into(),
            width: 100.0,
            height: 100.0,
            rotation: 0.0,

            fill_color: "#ffffff".into(),
            fill_opacity: 1.0,

            stroke_color: "#000000".into(),
            stroke_width: 1.0,
            stroke_opacity: 1.0,
            stroke_dash: vec![],

            border_radius: 0.0,

            shadow_color: "rgba(0,0,0,0)".into(),
            shadow_blur: 0.0,
            shadow_offset_x: 0.0,
            shadow_offset_y: 0.0,

            global_alpha: 1.0,
            composite_operation: "source-over".into(),
            stroke_style: "Solid".into()
        }
    }
}

#[derive(Clone, Debug,Default, Store)]
pub struct FormContext {
    pub TextForm: TypographySettings,
    pub ShapeForm: ShapeSettings
}

#[component]
pub fn FormProvider(){
    provide_context(Store::new(FormContext::default()));
}


pub fn use_form_context()-> Store<FormContext> {
    let ctx = expect_context::<Store<FormContext>>();
    ctx
}


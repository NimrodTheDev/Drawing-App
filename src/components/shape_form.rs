use leptos::prelude::*;
use crate::layout::form_context::use_form_context;
use crate::layout::form_context::FormContextStoreFields;


pub const SHAPES: &[&str] = &[
    "rect",
    "circle",
    "ellipse",
    "triangle",
    "diamond",
    "pentagon",
    "hexagon",
    "octagon",
    "star",
];

#[component]
pub fn ShapeForm() -> impl IntoView {
    let form_context = use_form_context();
    let settings = form_context.ShapeForm();

    view! {
        <div class="flex flex-col gap-3 p-4">

        <div class="flex flex-col gap-1">
            <label class="text-xs text-zinc-400">"Shape"</label>

            <div class="grid grid-cols-3 gap-1">
                {
                    SHAPES.into_iter().map(|shape| {
                        let shape_name = shape.to_string();

                        view! {
                            <div
                                class={
                                    let shape_name = shape_name.clone();
                                    move || format!(
                                        "py-1.5 px-2 flex items-center justify-center rounded-md border text-xs cursor-pointer transition-colors {}",
                                        if settings.get().shape == shape_name.clone() {
                                            "border-button text-button bg-button/10"
                                        } else {
                                            "border-white/10 text-zinc-400 hover:border-white/20 hover:text-white"
                                        }
                                    )}
                                on:click={
                                    let shape_name = shape_name.clone();
                                    move |_| {
                                        settings.update(|s| {
                                            s.shape = shape_name.clone();
                                        });
                                    }
                                }
                            >
                                {shape_name.clone()}
                            </div>
                        }
                    }).collect_view()
                }
            </div>
        </div>

        <div class="border-t border-white/10 my-1"></div>

            // Dimensions
            <div class="flex flex-col gap-1">
                <div class="flex justify-between items-center">
                    <label class="text-xs text-zinc-400">"Width"</label>
                    <span class="text-xs text-white/60 tabular-nums">
                        {move || format!("{}px", settings.get().width as i32)}
                    </span>
                </div>
                <input
                    type="range" min="0" max="1000" step="1"
                    value=move || settings.get().width as i32
                    class="w-full accent-button"
                    on:input=move |e| {
                        let v = event_target_value(&e).parse::<f64>().unwrap_or(100.0);
                        settings.update(|s| s.width = v);
                    }
                />
            </div>

            <div class="flex flex-col gap-1">
                <div class="flex justify-between items-center">
                    <label class="text-xs text-zinc-400">"Height"</label>
                    <span class="text-xs text-white/60 tabular-nums">
                        {move || format!("{}px", settings.get().height as i32)}
                    </span>
                </div>
                <input
                    type="range" min="0" max="1000" step="1"
                    value=move || settings.get().height as i32
                    class="w-full accent-button"
                    on:input=move |e| {
                        let v = event_target_value(&e).parse::<f64>().unwrap_or(100.0);
                        settings.update(|s| s.height = v);
                    }
                />
            </div>

            <div class="flex flex-col gap-1">
                <div class="flex justify-between items-center">
                    <label class="text-xs text-zinc-400">"Rotation"</label>
                    <span class="text-xs text-white/60 tabular-nums">
                        {move || format!("{}°", settings.get().rotation as i32)}
                    </span>
                </div>
                <input
                    type="range" min="0" max="360" step="1"
                    value=move || settings.get().rotation as i32
                    class="w-full accent-button"
                    on:input=move |e| {
                        let v = event_target_value(&e).parse::<f64>().unwrap_or(0.0);
                        settings.update(|s| s.rotation = v);
                    }
                />
            </div>

            <div class="flex flex-col gap-1">
                <div class="flex justify-between items-center">
                    <label class="text-xs text-zinc-400">"Border radius"</label>
                    <span class="text-xs text-white/60 tabular-nums">
                        {move || format!("{}px", settings.get().border_radius as i32)}
                    </span>
                </div>
                <input
                    type="range" min="0" max="200" step="1"
                    value=move || settings.get().border_radius as i32
                    class="w-full accent-button"
                    on:input=move |e| {
                        let v = event_target_value(&e).parse::<f64>().unwrap_or(0.0);
                        settings.update(|s| s.border_radius = v);
                    }
                />
            </div>

            // Divider
            <div class="border-t border-white/10 my-1"></div>

            // Fill
            <div class="flex flex-col gap-1">
                <div class="flex justify-between items-center">
                    <label class="text-xs text-zinc-400">"Fill color"</label>
                    <span
                        class="w-5 h-5 rounded-full border border-white/10"
                        style=move || format!("background: {}", settings.get().fill_color)
                    ></span>
                </div>
                <input
                    type="color"
                    value=move || settings.get().fill_color.clone()
                    class="w-full h-8 rounded-md cursor-pointer bg-transparent"
                    on:input=move |e| {
                        let v = event_target_value(&e);
                        settings.update(|s| s.fill_color = v);
                    }
                />
            </div>

            <div class="flex flex-col gap-1">
                <div class="flex justify-between items-center">
                    <label class="text-xs text-zinc-400">"Fill opacity"</label>
                    <span class="text-xs text-white/60 tabular-nums">
                        {move || format!("{:.0}%", settings.get().fill_opacity * 100.0)}
                    </span>
                </div>
                <input
                    type="range" min="0" max="1" step="0.01"
                    value=move || settings.get().fill_opacity
                    class="w-full accent-button"
                    on:input=move |e| {
                        let v = event_target_value(&e).parse::<f64>().unwrap_or(1.0);
                        settings.update(|s| s.fill_opacity = v);
                    }
                />
            </div>

            // Divider
            <div class="border-t border-white/10 my-1"></div>

            // Stroke
            <div class="flex flex-col gap-1">
                <label class="text-xs text-zinc-400">"Stroke"</label>
                <div class="grid grid-cols-3 gap-1">
                    <div
                        class=move || format!(
                            "py-1.5 flex items-center justify-center rounded-md border text-sm transition-colors {}",
                            if settings.get().stroke_width > 0.0 {
                                "border-button text-button bg-button/10"
                            } else {
                                "border-white/10 text-zinc-400 hover:border-white/20 hover:text-white"
                            }
                        )
                        on:click=move |_| settings.update(|s| {
                            s.stroke_width = if s.stroke_width > 0.0 { 0.0 } else { 1.0 };
                        })
                    >
                        <i class="bi bi-border-outer"></i>
                    </div>
                    {move || (settings.get().stroke_width > 0.0).then_some(view! {
                        <div class="col-span-2">
                            <input
                                type="color"
                                value=move || settings.get().stroke_color.clone()
                                class="w-full h-full rounded-md cursor-pointer bg-transparent"
                                on:input=move |e| {
                                    let v = event_target_value(&e);
                                    settings.update(|s| s.stroke_color = v);
                                }
                            />
                        </div>
                    })}
                </div>
            </div>

            {move || (settings.get().stroke_width > 0.0).then_some(view! {
                <div class="flex flex-col gap-1">
                    <div class="flex justify-between items-center">
                        <label class="text-xs text-zinc-400">"Stroke width"</label>
                        <span class="text-xs text-white/60 tabular-nums">
                            {move || format!("{}px", settings.get().stroke_width as i32)}
                        </span>
                    </div>
                    <input
                        type="range" min="0" max="20" step="0.5"
                        value=move || settings.get().stroke_width
                        class="w-full accent-button"
                        on:input=move |e| {
                            let v = event_target_value(&e).parse::<f64>().unwrap_or(1.0);
                            settings.update(|s| s.stroke_width = v);
                        }
                    />
                </div>

                // Stroke style
                <div class="flex flex-col gap-1">
                    <label class="text-xs text-zinc-400">"Stroke style"</label>
                    <div class="grid grid-cols-3 gap-1">
                        {[("Solid", "solid"), ("Dashed", "dashed"), ("Dotted", "dotted")].into_iter().map(|(label, value)| {
                            view! {
                                <div
                                    class=move || format!(
                                        "py-1.5 flex items-center justify-center rounded-md border text-xs transition-colors {}",
                                        if settings.get().stroke_style == value {
                                            "border-button text-button bg-button/10"
                                        } else {
                                            "border-white/10 text-zinc-400 hover:border-white/20 hover:text-white"
                                        }
                                    )
                                    on:click=move |_| settings.update(|s| s.stroke_style = value.into())
                                >
                                    {label}
                                </div>
                            }
                        }).collect_view()}
                    </div>
                </div>
            })}

            // Divider
            <div class="border-t border-white/10 my-1"></div>

            // Shadow
            <div class="flex flex-col gap-1">
                <label class="text-xs text-zinc-400">"Shadow"</label>
                <div
                    class=move || format!(
                        "py-1.5 flex items-center justify-center rounded-md border text-sm transition-colors {}",
                        if settings.get().shadow_blur > 0.0 {
                            "border-button text-button bg-button/10"
                        } else {
                            "border-white/10 text-zinc-400 hover:border-white/20 hover:text-white"
                        }
                    )
                    on:click=move |_| settings.update(|s| {
                        s.shadow_blur = if s.shadow_blur > 0.0 { 0.0 } else { 10.0 };
                    })
                >
                    <i class="bi bi-shadows"></i>
                </div>
            </div>

            {move || (settings.get().shadow_blur > 0.0).then_some(view! {
                <div class="flex flex-col gap-1">
                    <div class="flex justify-between items-center">
                        <label class="text-xs text-zinc-400">"Shadow blur"</label>
                        <span class="text-xs text-white/60 tabular-nums">
                            {move || format!("{}px", settings.get().shadow_blur as i32)}
                        </span>
                    </div>
                    <input
                        type="range" min="0" max="50" step="1"
                        value=move || settings.get().shadow_blur
                        class="w-full accent-button"
                        on:input=move |e| {
                            let v = event_target_value(&e).parse::<f64>().unwrap_or(0.0);
                            settings.update(|s| s.shadow_blur = v);
                        }
                    />
                </div>

                <div class="grid grid-cols-2 gap-2">
                    <div class="flex flex-col gap-1">
                        <div class="flex justify-between items-center">
                            <label class="text-xs text-zinc-400">"Offset X"</label>
                            <span class="text-xs text-white/60 tabular-nums">
                                {move || format!("{}px", settings.get().shadow_offset_x as i32)}
                            </span>
                        </div>
                        <input
                            type="range" min="-50" max="50" step="1"
                            value=move || settings.get().shadow_offset_x
                            class="w-full accent-button"
                            on:input=move |e| {
                                let v = event_target_value(&e).parse::<f64>().unwrap_or(0.0);
                                settings.update(|s| s.shadow_offset_x = v);
                            }
                        />
                    </div>
                    <div class="flex flex-col gap-1">
                        <div class="flex justify-between items-center">
                            <label class="text-xs text-zinc-400">"Offset Y"</label>
                            <span class="text-xs text-white/60 tabular-nums">
                                {move || format!("{}px", settings.get().shadow_offset_y as i32)}
                            </span>
                        </div>
                        <input
                            type="range" min="-50" max="50" step="1"
                            value=move || settings.get().shadow_offset_y
                            class="w-full accent-button"
                            on:input=move |e| {
                                let v = event_target_value(&e).parse::<f64>().unwrap_or(0.0);
                                settings.update(|s| s.shadow_offset_y = v);
                            }
                        />
                    </div>
                </div>

                <div class="flex flex-col gap-1">
                    <div class="flex justify-between items-center">
                        <label class="text-xs text-zinc-400">"Shadow color"</label>
                        <span
                            class="w-5 h-5 rounded-full border border-white/10"
                            style=move || format!("background: {}", settings.get().shadow_color)
                        ></span>
                    </div>
                    <input
                        type="color"
                        value=move || settings.get().shadow_color.clone()
                        class="w-full h-8 rounded-md cursor-pointer bg-transparent"
                        on:input=move |e| {
                            let v = event_target_value(&e);
                            settings.update(|s| s.shadow_color = v);
                        }
                    />
                </div>
            })}

            // Divider
            <div class="border-t border-white/10 my-1"></div>

            // Blend mode
            <div class="flex flex-col gap-1">
                <label class="text-xs text-zinc-400">"Blend mode"</label>
                <select
                    class="bg-aux border border-white/10 rounded-md px-2 py-1.5 text-sm text-white"
                    on:change=move |e| {
                        let v = event_target_value(&e);
                        settings.update(|s| s.composite_operation = v);
                    }
                >
                    {[
                        ("Normal",   "source-over"),
                        ("Multiply", "multiply"),
                        ("Screen",   "screen"),
                        ("Overlay",  "overlay"),
                        ("Darken",   "darken"),
                        ("Lighten",  "lighten"),
                        ("Eraser",   "destination-out"),
                    ].into_iter().map(|(label, value)| view! {
                        <option value=value>{label}</option>
                    }).collect_view()}
                </select>
            </div>

            // Global opacity
            <div class="flex flex-col gap-1">
                <div class="flex justify-between items-center">
                    <label class="text-xs text-zinc-400">"Opacity"</label>
                    <span class="text-xs text-white/60 tabular-nums">
                        {move || format!("{:.0}%", settings.get().global_alpha * 100.0)}
                    </span>
                </div>
                <input
                    type="range" min="0" max="1" step="0.01"
                    value=move || settings.get().global_alpha
                    class="w-full accent-button"
                    on:input=move |e| {
                        let v = event_target_value(&e).parse::<f64>().unwrap_or(1.0);
                        settings.update(|s| s.global_alpha = v);
                    }
                />
            </div>

        </div>
    }
}
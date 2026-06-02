use leptos::prelude::*;
use crate::layout::form_context::use_form_context;
use crate::layout::form_context::FormContextStoreFields;

#[component]
pub fn TypographyForm() -> impl IntoView {
    let form_context = use_form_context();
    let settings = form_context.TextForm();


    view! {
    <div class="flex flex-col gap-3 p-4">

        // Font family
        <div class="flex flex-col gap-1">
            <label class="text-xs text-zinc-400">"Font family"</label>
            <select
                class="bg-aux border border-white/10 rounded-md px-2 py-1.5 text-sm text-white"
                on:change=move |e| {
                    let v = event_target_value(&e);
                    settings.update(|s| s.font_family = v);
                }
            >
                <option value="sans">"Sans-serif"</option>
                <option value="serif">"Serif"</option>
                <option value="mono">"Monospace"</option>
            </select>
        </div>

        // Font weight
        <div class="flex flex-col gap-1">
            <label class="text-xs text-zinc-400">"Font weight"</label>
            <select
                class="bg-aux border border-white/10 rounded-md px-2 py-1.5 text-sm text-white"
                on:change=move |e| {
                    let v = event_target_value(&e);
                    settings.update(|s| s.font_weight = v);
                }
            >
                <option value="300">"Light — 300"</option>
                <option value="400" selected>"Regular — 400"</option>
                <option value="500">"Medium — 500"</option>
                <option value="600">"Semibold — 600"</option>
                <option value="700">"Bold — 700"</option>
            </select>
        </div>

        // Font size
        <div class="flex flex-col gap-1">
            <div class="flex justify-between items-center">
                <label class="text-xs text-zinc-400">"Font size"</label>
                <span class="text-xs text-white/60 tabular-nums">
                    {move || format!("{}px", settings.get().font_size as i32)}
                </span>
            </div>
            <input
                type="range" min="10" max="72" step="1"
                value=move || settings.get().font_size as i32
                class="w-full accent-button"
                on:input=move |e| {
                    let v = event_target_value(&e).parse::<f64>().unwrap_or(16.0);
                    settings.update(|s| s.font_size = v);
                }
            />
        </div>

        // Line height

        // Letter spacing


        <div class="flex flex-col gap-1">
            <div class="flex justify-between items-center">
                <label class="text-xs text-zinc-400">"Color"</label>
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
        // Divider
        <div class="border-t border-white/10 my-1"></div>

        // Text align
        <div class="flex flex-col gap-1">
            <label class="text-xs text-zinc-400">"Alignment"</label>
            <div class="grid grid-cols-4 gap-1">
                {["left", "center", "right", "justify"].into_iter().map(|align| {
                    let icon = match align {
                        "left"    => "bi bi-text-left",
                        "center"  => "bi bi-text-center",
                        "right"   => "bi bi-text-right",
                        _         => "bi bi-justify",
                    };
                    view! {
                        <div
                            class=move || {
                                let active = settings.get().text_align == align;
                                format!(
                                    "py-1.5 rounded-md flex item-center justify-center text-sm transition-colors {}",
                                    if active {
                                        "border-button text-zinc-400 bg-button"
                                    } else {
                                        "border-white/10 text-zinc-400 hover:border-white/20 hover:text-white"
                                    }
                                )
                            }
                            on:click={
                                    move|_| settings.update(|s| {
                                        s.text_align = align.into();
                                    })
                                }
                        >

                        <button
                        >
                            <i class=icon></i>
                        </button>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>

        // Style toggles
        <div class="flex flex-col gap-1">
            <label class="text-xs text-zinc-400">"Style"</label>
            <div class="grid grid-cols-3 gap-1">
                <div
                    class=move || format!(
                        "py-1.5 flex items-center justify-center rounded-md border text-sm italic transition-colors {}",
                        if settings.get().italic {
                            "border-button text-button bg-button/10"
                        } else {
                            "border-white/10 text-zinc-400 hover:border-white/20 hover:text-white"
                        }
                    )
                    on:click=move |_| settings.update(|s| s.italic = !s.italic)
                >
                    <i class="bi bi-type-italic"></i>
                </div>
                <div
                    class=move || format!(
                        "py-1.5 rounded-md border flex items-center justify-center text-sm line-through transition-colors {}",
                        if settings.get().stroke_text {
                            "border-button text-button bg-button/10"
                        } else {
                            "border-white/10 text-zinc-400 hover:border-white/20 hover:text-white"
                        }
                    )
                    on:click=move |_| settings.update(|s| s.stroke_text = !s.stroke_text)
                >
                    <i class="bi bi-fonts"></i>
                </div>
                {move|| settings.get().stroke_text.then_some(
                    Some(view! {
                        <button
                            class="py-1.5 rounded-md flex items-center justify-center border text-sm line-through transition-colors"
                        >
                            <input
                                type="color"
                                value=move || settings.get().stroke_color.clone()
                                class="w-full h-8 rounded-md cursor-pointer bg-transparent"
                                on:input=move |e| {
                                    let v = event_target_value(&e);
                                    settings.update(|s| s.stroke_color = v);
                                }
                            />
                        </button>
                    })
                )}
            </div>
        </div>

    </div>
}
}
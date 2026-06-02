use leptos::prelude::*;

#[derive(Clone)]
pub struct ThemeContext {
    pub is_dark: ReadSignal<bool>,
    pub set_is_dark: WriteSignal<bool>,
}

#[component]
pub fn ThemeContext(
    children: Children
)-> impl IntoView {
    let (is_dark, set_is_dark)  = signal(false);
    provide_context(
        ThemeContext {
            is_dark,
            set_is_dark
        }
    );

    Effect::new(move |context| {
        let window = leptos::web_sys::window().expect("failed to create web window");
        let document = window.document().expect("failed to get window document");
        let html = document.document_element().expect("failed to get document element");
        html.set_class_name(if is_dark.get() { "dark" } else { "light" });
    });
    view! {
        <div class="flex gap-2 flex-col h-full">
        {children()}
        </div>
    }
}
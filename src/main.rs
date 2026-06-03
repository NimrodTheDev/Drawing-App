pub mod views;
pub mod components;
pub mod layout;
pub mod art_engine;

use leptos::*;
use leptos::html::base;
use leptos::prelude::*;
use leptos_router::*;
use leptos_router::components::*;
use crate::views::{CreativeHub, NavBar, draw};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! {
        <App/>
    });
}

#[component]
fn App() -> impl IntoView {

    view! {
        <Router >
            <layout::theme_context::ThemeContext>
                <NavBar/>
                <Routes  fallback=||"No view">
                    <Route path=path!("/") view=CreativeHub/>
                    <Route path=path!("/Drawing-App/") view=CreativeHub/>
                    <Route path=path!("/Drawing-App/art") view=move || view!{<layout::art_layout::ArtLayout>
                <draw::DrawView/>
            </layout::art_layout::ArtLayout>}/>
                </Routes>
            </layout::theme_context::ThemeContext>
        </Router>
    }
}

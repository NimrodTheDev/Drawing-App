pub mod draw;

use leptos::*;
use leptos::prelude::*;
use leptos::web_sys::js_sys::Math::exp;
use leptos_router::hooks::use_location;
// ── Data model ────────────────────────────────────────────────────────────────

#[derive(Clone)]
struct Project {
    id: u32,
    title: &'static str,
    edited: &'static str,
    thumb_bg: &'static str,   // Tailwind gradient classes
    thumb_icon: ThumbnailIcon,
}

#[derive(Clone, Copy)]
enum ThumbnailIcon {
    Grid,
    BarChart,
    Rings,
    LineChart,
    Hexagon,
    Browser,
}

fn projects() -> Vec<Project> {
    vec![
        Project {
            id: 1,
            title: "Brand Identity Redesign",
            edited: "Edited 2 hours ago",
            thumb_bg: "from-pink-100 to-pink-200",
            thumb_icon: ThumbnailIcon::Grid,
        },
        Project {
            id: 2,
            title: "Q3 Pitch Deck",
            edited: "Edited yesterday",
            thumb_bg: "from-blue-100 to-blue-200",
            thumb_icon: ThumbnailIcon::BarChart,
        },
        Project {
            id: 3,
            title: "User Research Summary",
            edited: "Edited 3 days ago",
            thumb_bg: "from-green-100 to-green-200",
            thumb_icon: ThumbnailIcon::Rings,
        },
        Project {
            id: 4,
            title: "Marketing Analytics",
            edited: "Edited 5 days ago",
            thumb_bg: "from-amber-100 to-amber-200",
            thumb_icon: ThumbnailIcon::LineChart,
        },
        Project {
            id: 5,
            title: "Product Roadmap 2026",
            edited: "Edited last week",
            thumb_bg: "from-violet-100 to-violet-200",
            thumb_icon: ThumbnailIcon::Hexagon,
        },
        Project {
            id: 6,
            title: "Website Wireframes",
            edited: "Edited 2 weeks ago",
            thumb_bg: "from-teal-100 to-teal-200",
            thumb_icon: ThumbnailIcon::Browser,
        },
    ]
}

// ── SVG icon helpers ──────────────────────────────────────────────────────────

#[component]
fn IconSearch() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none"
            viewBox="0 0 16 16" stroke="currentColor" stroke-width="1.5">
            <circle cx="7" cy="7" r="5"/>
            <path d="M11 11l3 3"/>
        </svg>
    }
}

#[component]
fn IconGrid() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 16 16" fill="currentColor">
            <rect x="1" y="1" width="6" height="6" rx="1.5"/>
            <rect x="9" y="1" width="6" height="6" rx="1.5"/>
            <rect x="1" y="9" width="6" height="6" rx="1.5"/>
            <rect x="9" y="9" width="6" height="6" rx="1.5"/>
        </svg>
    }
}

#[component]
fn IconSettings() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none"
            viewBox="0 0 16 16" stroke="currentColor" stroke-width="1.5">
            <circle cx="8" cy="8" r="2.5"/>
            <path d="M8 1v2M8 13v2M1 8h2M13 8h2M3.05 3.05l1.41 1.41M11.54 11.54l1.41 1.41M3.05 12.95l1.41-1.41M11.54 4.46l1.41-1.41"/>
        </svg>
    }
}

#[component]
fn IconPlus() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none"
            viewBox="0 0 14 14" stroke="white" stroke-width="2">
            <path d="M7 1v12M1 7h12"/>
        </svg>
    }
}

#[component]
fn IconImport() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none"
            viewBox="0 0 14 14" stroke="currentColor" stroke-width="1.5">
            <path d="M7 1v8M4 6l3 3 3-3"/>
            <path d="M2 10v2a1 1 0 001 1h8a1 1 0 001-1v-2"/>
        </svg>
    }
}

#[component]
fn IconDots() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" class="w-3 h-3" viewBox="0 0 12 12" fill="white">
            <circle cx="6" cy="2" r="1.2"/>
            <circle cx="6" cy="6" r="1.2"/>
            <circle cx="6" cy="10" r="1.2"/>
        </svg>
    }
}

// ── Thumbnail SVG artwork ─────────────────────────────────────────────────────

#[component]
fn ThumbnailArt(icon: ThumbnailIcon) -> impl IntoView {
    match icon {
        ThumbnailIcon::Grid => view! {
            <svg class="w-14 h-14 opacity-30" viewBox="0 0 60 60" fill="#be185d">
                <rect x="5" y="5" width="22" height="22" rx="3"/>
                <rect x="33" y="5" width="22" height="22" rx="3"/>
                <rect x="5" y="33" width="22" height="22" rx="3"/>
                <rect x="33" y="33" width="22" height="22" rx="3"/>
            </svg>
        }.into_any(),
        ThumbnailIcon::BarChart => view! {
            <svg class="w-14 h-10 opacity-30" viewBox="0 0 60 40" fill="#1d4ed8">
                <rect x="0" y="28" width="8" height="12"/>
                <rect x="13" y="18" width="8" height="22"/>
                <rect x="26" y="10" width="8" height="30"/>
                <rect x="39" y="20" width="8" height="20"/>
                <rect x="52" y="5" width="8" height="35"/>
            </svg>
        }.into_any(),
        ThumbnailIcon::Rings => view! {
            <svg class="w-14 h-14 opacity-30" viewBox="0 0 52 52" fill="none" stroke="#15803d" stroke-width="4">
                <circle cx="26" cy="26" r="22"/>
                <circle cx="26" cy="26" r="12"/>
                <circle cx="26" cy="26" r="5" fill="#15803d" stroke="none"/>
            </svg>
        }.into_any(),
        ThumbnailIcon::LineChart => view! {
            <svg class="w-14 h-10 opacity-30" viewBox="0 0 58 44" fill="none" stroke="#b45309" stroke-width="3">
                <path d="M4 36 L18 16 L32 24 L46 8"/>
                <circle cx="18" cy="16" r="4" fill="#b45309" stroke="none"/>
                <circle cx="32" cy="24" r="4" fill="#b45309" stroke="none"/>
                <circle cx="46" cy="8" r="4" fill="#b45309" stroke="none"/>
            </svg>
        }.into_any(),
        ThumbnailIcon::Hexagon => view! {
            <svg class="w-14 h-14 opacity-30" viewBox="0 0 56 56" fill="none" stroke="#6d28d9" stroke-width="2.5">
                <polygon points="28,4 52,18 52,38 28,52 4,38 4,18"/>
                <polygon points="28,14 42,22 42,34 28,42 14,34 14,22"/>
                <circle cx="28" cy="28" r="6" fill="#6d28d9" stroke="none"/>
            </svg>
        }.into_any(),
        ThumbnailIcon::Browser => view! {
            <svg class="w-14 h-10 opacity-30" viewBox="0 0 56 40" fill="none" stroke="#0f766e" stroke-width="2.5">
                <rect x="4" y="4" width="48" height="32" rx="4"/>
                <path d="M4 14h48"/>
                <circle cx="14" cy="9" r="2.5" fill="#0f766e" stroke="none"/>
                <circle cx="22" cy="9" r="2.5" fill="#0f766e" stroke="none"/>
            </svg>
        }.into_any(),
    }
}

// ── Project card ──────────────────────────────────────────────────────────────

#[component]
fn ProjectCard(project: Project) -> impl IntoView {
    let (hovered, set_hovered) = signal(false);

    view! {
        <div
            class="group bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800
                   overflow-hidden cursor-pointer transition-all duration-200
                   hover:-translate-y-1 hover:border-zinc-300 dark:hover:border-zinc-700
                   hover:shadow-md"
            on:mouseenter=move |_| set_hovered.set(true)
            on:mouseleave=move |_| set_hovered.set(false)
        >
            // Thumbnail
            <div class=format!(
                "h-36 bg-gradient-to-br {} relative flex items-center justify-center",
                project.thumb_bg
            )>
                <ThumbnailArt icon=project.thumb_icon />

                // Context menu button — visible on hover
                <button
                    class=move || format!(
                        "absolute top-2 right-2 w-7 h-7 rounded-lg bg-black/40 flex items-center
                         justify-center transition-opacity duration-150 {}",
                        if hovered.get() { "opacity-100" } else { "opacity-0" }
                    )
                    on:click=|ev| ev.stop_propagation()
                >
                    <IconDots />
                </button>
            </div>

            // Card body
            <div class="px-3.5 py-3">
                <p class="text-sm font-medium text-zinc-900 dark:text-zinc-100 truncate">
                    {project.title}
                </p>
                <p class="text-xs text-zinc-400 dark:text-zinc-500 mt-0.5">
                    {project.edited}
                </p>
            </div>
        </div>
    }
}

// ── Top navigation ────────────────────────────────────────────────────────────

use leptos_router::*;
use crate::layout::theme_context::ThemeContext;

#[component]
pub fn NavBar() -> impl IntoView {
    let location = use_location();

    let breadcrumb = move || {
        let path = location.pathname.get();

        if path == "/" {
            "Home".to_string()
        } else {
            path.split('/')
                .filter(|s| !s.is_empty())
                .map(|s| {
                    let mut c = s.chars();
                    match c.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                    }
                })
                .collect::<Vec<_>>()
                .join(" / ")
        }
    };
    let theme_context: ThemeContext = expect_context();
    view! {
        <nav class="h-14 px-5 flex items-center justify-between
                    bg-[#151821] backdrop-blur-md
                    border-b border-[#232735]">

            // LEFT
            <div class="flex items-center gap-4">
                <div class="text-indigo-400 font-semibold tracking-wide">
                    "CreativeHub"
                </div>

                <div class="text-sm text-gray-400">
                    {breadcrumb}
                </div>
            </div>

            // CENTER
            <div class="flex-1 flex justify-center">
                <input
                    type="text"
                    placeholder="Search projects, assets..."
                    class="w-80 max-w-full
                           bg-[#1b2030]
                           border border-[#2a2f3f]
                           text-gray-200 text-sm
                           px-3 py-2 rounded-lg
                           outline-none
                           focus:border-indigo-500
                           focus:ring-2 focus:ring-indigo-500/20
                           transition"
                />
            </div>

            // RIGHT
            <div class="flex items-center">
                <button
                    class="bg-[#1b2030]
                           border border-[#2a2f3f]
                           text-gray-200
                            text-aux
                           px-3 py-2 rounded-lg
                           hover:bg-[#20263a]
                           hover:border-indigo-500
                           transition"
                on:click=move|_|{
                    theme_context.set_is_dark.set(!theme_context.is_dark.get());
            }
                >
                    {move|| {
                        if theme_context.is_dark.get()  {
                             view! {
                                    <i class="bi bi-sunrise w-9 h-9"></i>
                                }
                            }else{
                                view! {
                                    <i class="bi bi-moon-stars-fill"></i>}
                                }
                        }
                    }

                </button>
            </div>
        </nav>
    }
}
// ── Main page ─────────────────────────────────────────────────────────────────

use leptos_router::components::A;
#[component]
pub fn CreativeHub() -> impl IntoView {
    let project_list = projects();

    view! {
        <div class="min-h-screen bg-zinc-50 dark:bg-zinc-950 font-sans">
            // <NavBar />

            <main class="max-w-5xl mx-auto px-6 pt-10 pb-16">

                // Hero
                <div class="mb-8">
                    <h1 class="font-serif text-3xl font-normal tracking-tight
                               text-zinc-950 dark:text-zinc-100 mb-1">
                        "Welcome back"
                    </h1>
                    <p class="text-sm text-zinc-500 dark:text-zinc-400">
                        "Create, edit, and bring your ideas to life"
                    </p>
                </div>

                // Action buttons
                <div class="flex gap-3 mb-10">
                    <A  href="/Drawing-App/art" attr:class="flex items-center gap-2 px-5 h-11 rounded-xl bg-pink-500
                                   hover:bg-pink-600 active:scale-[.98] text-white text-sm
                                   font-medium transition-all duration-150 shadow-sm">
                        <IconPlus />
                        "Start New Project"
                    </A>
                    <button class="flex items-center gap-2 px-5 h-11 rounded-xl bg-white
                                   dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-700
                                   hover:bg-zinc-50 dark:hover:bg-zinc-800 active:scale-[.98]
                                   text-zinc-800 dark:text-zinc-200 text-sm font-medium
                                   transition-all duration-150">
                        <IconImport />
                        "Import"
                    </button>
                </div>

                // Section label
                <p class="text-xs font-medium text-zinc-400 dark:text-zinc-500 tracking-widest
                           uppercase mb-4">
                    "Recent projects"
                </p>

                // Project grid
                <div class="grid grid-cols-[repeat(auto-fill,minmax(200px,1fr))] gap-4">
                    <For
                        each=move || project_list.clone()
                        key=|p| p.id
                        children=|project| view! { <ProjectCard project=project /> }
                    />
                </div>
            </main>
        </div>
    }
}



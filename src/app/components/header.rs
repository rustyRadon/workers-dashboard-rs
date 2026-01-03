use leptos::prelude::*;
use leptos_router::components::*; 
use leptos_router::hooks::use_location; 

const INPUT_STYLE: &str = 
    "border-b-0 border-[#7734e7] h-8 text-white ml-4 mr-4 hover:border-b-2";
const INPUT_STYLE_SELECTED: &str =
    "border-b-2 border-[#9734e7] h-8 text-white ml-4 mr-4 hover:border-b-2";

#[component]
pub fn Header() -> impl IntoView {
    let location = use_location();
    
    Effect::new(move |_| {
        leptos::logging::log!("Current path: {}", location.pathname.get());
    });

    let is_dashboard = Memo::new(move |_| {
        let path = location.pathname.get();
        path == "/" || path.is_empty()
    });
    let is_team = Memo::new(move |_| location.pathname.get() == "/team");

    view! {
        <div class="flex mx-auto items-center justify-center w-full h-16 fixed top-0 bg-gray-800 z-50 shadow-md">
            <nav class="flex flex-row items-center justify-center h-full">

                <div class=move || nav_style(is_dashboard.get())>
                    <A href="/">"Dashboard"</A>
                </div>

                <div class=move || nav_style(is_team.get())>
                    <A href="/team">"Team"</A>
                </div>

            </nav>
        </div>
    }
}

fn nav_style(is_active: bool) -> &'static str {
    if is_active {
        INPUT_STYLE_SELECTED
    } else {
        INPUT_STYLE
    }
}
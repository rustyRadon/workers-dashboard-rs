use leptos::prelude::*;
use leptos_router::components::*; 
use leptos_router::hooks::use_location; 

const INPUT_STYLE: &str = 
    "border-b-0 border-[#7734e7] h-8 text-white ml-4 mr-4 hover:border-b-2";
const INPUT_STYLE_SELECTED: &str =
    "border-b-2 border-[#9734e7] h-8 text-white ml-4 mr-4 hover:border-b-2";

#[component]
pub fn Header() -> impl IntoView {
    // Get the current location object
    let location = use_location();
    

    let is_dashboard = Memo::new(move |_| location.pathname.get() == "/");
    let is_team = Memo::new(move |_| location.pathname.get() == "/team");

    view! {
        <div class="flex mx-auto items-center w-full h-12 pt-8 px-20 fixed top-0">
            <nav class="flex flex-row w-full max-w-[52rem] h-12">

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
pub mod components;
pub mod pages;
pub use pages::{HomePage, TeamPage};
pub mod models;
pub mod server_functions;


use crate::app::components::Header; 
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
//use leptos_meta::Stylesheet;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        //<Stylesheet id="leptos" href="/pkg/dashboard-app-rs.css"/>
        <script src="https://cdn.tailwindcss.com"></script>
        <Title text="Staff dashboard app"/>

        <Router>
            <Header/>
            <main class="pt-20 bg-gray-900 min-h-screen">
                <Routes fallback=move || "Not found.">
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("team") view=TeamPage/>
                    <Route path=WildcardSegment("any") view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        if let Some(resp) = use_context::<leptos_actix::ResponseOptions>() {
            resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
        }
    }
    view! { <h1>"Not Found"</h1> }
}
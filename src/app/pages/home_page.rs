use crate::app::components::{DashboardChart, DashboardHeader, Header};
use crate::app::server_functions::persons::get_persons;
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let get_persons_info = Resource::new(|| (), |_| async move { get_persons().await });

    view! {
        <div class="w-full max-w-[64rem] mx-auto flex flex-col items-center justify-center">
            <Header />
            <DashboardHeader />
            <Suspense fallback=move || {
                view! { <p class="text-white">"Loading data..."</p> }
            }>
                {move || {
                    get_persons_info.get().map(|data| {
                        match data {
                            Ok(persons_data) => {
                                view! {
                                    <DashboardChart persons_data />
                                }.into_any() 
                            },
                            Err(_) => view! { <div></div> }.into_any() 
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}
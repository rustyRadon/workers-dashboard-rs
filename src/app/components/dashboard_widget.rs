use leptos::prelude::*;

#[component]
pub fn DashboardWidget(
    title: &'static str,
    #[prop(into)] value: String, // Use #[prop(into)] for maximum flexibility
) -> impl IntoView {
    view! {
        <div class="flex flex-col bg-[#38000A] rounded px-6 py-4">
            <p class="text-stone-400 text-sm uppercase font-semibold">{title}</p>
            <p class="text-white text-2xl font-bold mt-1">{value}</p>
        </div>
    }
}
use crate::app::{
    components::{AddPersonModal, Header, PersonRow, Toast, ToastMessage},
    server_functions::persons::get_persons,
};
use leptos::prelude::*;
use std::sync::Arc;

#[component]
pub fn TeamPage() -> impl IntoView {
    let if_show_modal = RwSignal::new(false);
    let if_show_toast = RwSignal::new(false);
    let toast_message = RwSignal::new(ToastMessage::new());
    let (search_query, set_search_query) = signal(String::new());

    let (refetch_trigger, set_refetch_trigger) = signal(());
    let get_persons_info = Resource::new(
        move || refetch_trigger.get(), 
        |_| async move { get_persons().await }
    );

    let filtered_persons = Memo::new(move |_| {
        let query = search_query.get().to_lowercase();
        let data = get_persons_info.get();
        
        match data {
            Some(Ok(persons)) => {
                if query.is_empty() {
                    persons
                } else {
                    persons.into_iter()
                        .filter(|p| {
                            p.name.to_lowercase().contains(&query) || 
                            p.title.to_lowercase().contains(&query)
                        })
                        .collect()
                }
            },
            _ => vec![],
        }
    });

    view! {
        <div class="w-full max-w-[64rem] mx-auto text-white">
            <Header />
            <Toast toast_message=toast_message if_appear=if_show_toast set_if_appear=if_show_toast.write_only() />

            <div class="mt-20 flex flex-col items-center">
                <Show when=move || if_show_modal.get()>
                    <AddPersonModal
                        set_if_show_modal=if_show_modal.write_only()
                        set_if_show_added=if_show_toast.write_only()
                        set_toast_message=toast_message.write_only()
                        set_refetch_trigger=set_refetch_trigger
                    />
                </Show>

                <div class="w-full max-w-[52rem] mb-10">
                <div class="relative flex items-center group">
                    <div class="absolute left-5 flex items-center justify-center pointer-events-none text-[#CD1C18]">
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
                        </svg>
                    </div>

                    <input 
                        type="text"
                        placeholder="Search team members..."
                        class="w-full bg-[#1e0005] border-2 border-[#CD1C18]/30 rounded-xl py-4 pl-14 pr-4 text-[#FFA896] placeholder:text-[#CD1C18]/40 focus:border-[#CD1C18] focus:ring-1 focus:ring-[#CD1C18] outline-none transition-all shadow-2xl"
                        on:input=move |ev| set_search_query.set(event_target_value(&ev))
                        prop:value=search_query
                    />
                </div>
            </div>
                <div class="flex flex-row w-full max-w-[52rem] items-center">
                    <div class="pr-4 text-xl font-bold text-[#FFA896]">"Members"</div>
                    <hr class="flex-grow border-[#CD1C18]/20 mx-4" />
                    <button 
                        on:click=move |_| if_show_modal.set(true) 
                        class="bg-[#CD1C18] hover:bg-[#9B1313] px-8 py-2 rounded-lg font-bold transition-colors shadow-lg"
                    >
                        "Add"
                    </button>
                </div>

                <Suspense fallback=move || view! { <p class="mt-10 text-[#FFA896] animate-pulse">"Accessing Database..."</p> }>
                    <div class="flex flex-col w-full max-w-[52rem] mt-6 mb-20">
                        {move || {
                            let results = filtered_persons.get();
                            if results.is_empty() && !search_query.get().is_empty() {
                                view! {
                                    <div class="text-center py-20 bg-[#1e0005] rounded-xl border border-dashed border-[#CD1C18]/30">
                                        <p class="text-[#CD1C18] text-lg font-medium">"No members found matching '" {search_query.get()} "'"</p>
                                    </div>
                                }.into_any()
                            } else {
                                results.into_iter().map(|each_person| {
                                    view! {
                                        <PersonRow
                                            person=Arc::new(each_person)
                                            set_refetch_trigger=set_refetch_trigger
                                            set_if_show_toast=if_show_toast.write_only()
                                            set_toast_message=toast_message.write_only()
                                        />
                                    }
                                }).collect_view().into_any()
                            }
                        }}
                    </div>
                </Suspense>
            </div>
        </div>
    }
}
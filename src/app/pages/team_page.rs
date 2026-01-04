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

    // Create the trigger and the Resource that tracks it
    let (refetch_trigger, set_refetch_trigger) = signal(());
    let get_persons_info = Resource::new(
        move || refetch_trigger.get(), 
        |_| async move { get_persons().await }
    );

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

                <div class="flex flex-row w-full max-w-[52rem] items-center">
                    <div class="pr-4 text-xl">"Members"</div>
                    <hr class="flex-grow border-stone-700 mx-4" />
                    <button on:click=move |_| if_show_modal.set(true) class="bg-[#38000A] px-8 py-2 rounded">"Add"</button>
                </div>

                <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                    <div class="flex flex-col w-full max-w-[52rem] mt-6">
                        {move || {
                            get_persons_info.get().map(|data| {
                                match data {
                                    Ok(persons_data) => {
                                        persons_data.into_iter().map(|each_person| {
                                            view! {
                                                <PersonRow
                                                    person=Arc::new(each_person)
                                                    set_refetch_trigger=set_refetch_trigger
                                                    set_if_show_toast=if_show_toast.write_only()
                                                    set_toast_message=toast_message.write_only()
                                                />
                                            }
                                        }).collect_view().into_any()
                                    },
                                    Err(_) => view! { <p>"Error loading members"</p> }.into_any()
                                }
                            })
                        }}
                    </div>
                </Suspense>
            </div>
        </div>
    }
}
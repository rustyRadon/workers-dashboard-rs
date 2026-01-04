use crate::app::components::{EditPersonModal, ShowPersonModal, toast::ToastMessage};
use crate::app::models::person::Person;
use leptos::prelude::*;
use std::sync::Arc;

#[component]
pub fn PersonRow(
    person: Arc<Person>,
    
    set_refetch_trigger: WriteSignal<()>, 
    set_if_show_toast: WriteSignal<bool>,
    set_toast_message: WriteSignal<ToastMessage>,
) -> impl IntoView {
    let if_show_info_modal = RwSignal::new(false);
    let if_show_edit_modal = RwSignal::new(false);

    let person_view = person.clone();
    let person_for_info = person.clone();
    let person_for_edit = person.clone();

    view! {
        <Show when=move || if_show_info_modal.get()>
            <ShowPersonModal
                person=person_for_info.clone()
                set_if_show_modal=if_show_info_modal.write_only()
                set_if_show_deleted=set_if_show_toast
                set_refetch_trigger=set_refetch_trigger 
                set_toast_message=set_toast_message
            />
        </Show>

        <Show when=move || if_show_edit_modal.get()>
            <EditPersonModal
                person=person_for_edit.clone()
                set_if_show_modal=if_show_edit_modal.write_only()
                set_if_show_toast=set_if_show_toast
                set_refetch_trigger=set_refetch_trigger 
                set_toast_message=set_toast_message
            />
        </Show>

        <div class="bg-[#283653] rounded px-10 py-5 mb-4 flex flex-row justify-between items-center text-white">
            <div class="flex flex-col">
                <p class="font-bold">{person_view.name.clone()}</p>
                <p class="text-sm text-stone-400">{person_view.title.clone()}</p>
            </div>
            <div class="flex flex-row gap-3">
                <button on:click=move |_| if_show_info_modal.set(true) class="border border-white rounded-full w-8 h-8">"i"</button>
                <button on:click=move |_| if_show_edit_modal.set(true)>
                    <img src="assets/edit.png" class="w-8 h-8" alt="Edit" />
                </button>
            </div>
        </div>
    }
}
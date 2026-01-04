use crate::app::components::toast::{ToastMessage, ToastMessageType};
use crate::app::models::person::{Person, DeletePersonRequest};
use crate::app::server_functions::persons::delete_person;
use leptos::prelude::*;
use leptos::task::spawn_local;
use std::sync::Arc;

#[component]
pub fn ShowPersonModal(
    person: Arc<Person>,
    set_if_show_modal: WriteSignal<bool>,
    set_if_show_deleted: WriteSignal<bool>,
    set_refetch_trigger: WriteSignal<()>,
    set_toast_message: WriteSignal<ToastMessage>,
) -> impl IntoView {
    let person_view = person.clone();
    let person_closure = person.clone();

    let on_delete = move |_| {
        let uuid = person_closure.uuid.clone();
        spawn_local(async move {
            let request = DeletePersonRequest::new(uuid);
            if delete_person(request).await.is_ok() {
                set_refetch_trigger.set(()); 
                set_if_show_modal.set(false);
                set_toast_message.set(ToastMessage::create(ToastMessageType::MemberDeleted));
                set_if_show_deleted.set(true);
            }
        });
    };

    view! {
        <div class="fixed inset-0 z-50 flex items-center justify-center bg-[#38000A]/60">
            <div class="bg-[#222222] border-t-8 border-red-500 p-8 w-full max-w-md text-white">
                <h2 class="text-2xl font-bold mb-2">{person_view.name.clone()}</h2>
                <p class="text-stone-400 mb-6">{person_view.title.clone()} " - " {person_view.level.clone()}</p>
                <div class="flex justify-between items-center">
                    <button class="bg-red-600 px-4 py-2 rounded hover:bg-red-700" on:click=on_delete>"Delete Member"</button>
                    <button class="text-stone-400 hover:text-white" on:click=move |_| set_if_show_modal.set(false)>"Close"</button>
                </div>
            </div>
        </div>
    }
}
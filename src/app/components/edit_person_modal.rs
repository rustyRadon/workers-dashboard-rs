use crate::app::components::toast::{ToastMessage, ToastMessageType};
use crate::app::models::person::{Person, EditPersonRequest};
use crate::app::server_functions::persons::edit_person;
use leptos::prelude::*;
use leptos::task::spawn_local;
use std::sync::Arc;
use validator::Validate;

#[component]
pub fn EditPersonModal(
    person: Arc<Person>,
    set_if_show_modal: WriteSignal<bool>,
    set_if_show_toast: WriteSignal<bool>,
    set_toast_message: WriteSignal<ToastMessage>,
    set_refetch_trigger: WriteSignal<()>,
) -> impl IntoView {
    let person_view = person.clone();
    let title = RwSignal::new(person.title.clone());
    let level = RwSignal::new(person.level.clone());
    let comp = RwSignal::new(person.compensation.to_string());
    
    let uuid = person.uuid.clone();
    let on_click = move |_| {
        if let Ok(c) = comp.get().parse::<i32>() {
            let req = EditPersonRequest::new(uuid.clone(), title.get(), level.get(), c);
            if req.validate().is_ok() {
                spawn_local(async move {
                    if edit_person(req).await.is_ok() {
                        set_refetch_trigger.set(());
                        set_if_show_modal.set(false);
                        set_toast_message.set(ToastMessage::create(ToastMessageType::MemberUpdated));
                        set_if_show_toast.set(true);
                    }
                });
            }
        }
    };

    view! {
        <div class="fixed inset-0 z-50 flex items-center justify-center bg-[#38000A]/60">
            <div class="bg-[#222222] border-t-8 border-[#7734e7] p-8 w-full max-w-md">
                <h2 class="text-white text-3xl mb-6">{person_view.name.clone()}</h2>
                <input type="text" class="w-full bg-[#333333] text-white p-3 mb-4" prop:value=move || title.get() on:input=move |e| title.set(event_target_value(&e)) />
                <input type="text" class="w-full bg-[#333333] text-white p-3 mb-4" prop:value=move || level.get() on:input=move |e| level.set(event_target_value(&e)) />
                <input type="text" class="w-full bg-[#333333] text-white p-3 mb-6" prop:value=move || comp.get() on:input=move |e| comp.set(event_target_value(&e)) />
                <div class="flex justify-end gap-4">
                    <button class="text-stone-400" on:click=move |_| set_if_show_modal.set(false)>"Cancel"</button>
                    <button class="bg-[#38000A] text-white px-6 py-2 rounded" on:click=on_click>"Update"</button>
                </div>
            </div>
        </div>
    }
}
use crate::app::components::toast::{ToastMessage, ToastMessageType};
use crate::app::models::person::AddPersonRequest;
use crate::app::server_functions::persons::add_person;
use leptos::prelude::*;
use leptos::task::spawn_local;
use validator::Validate;

#[component]
pub fn AddPersonModal(
    set_if_show_modal: WriteSignal<bool>,
    set_if_show_added: WriteSignal<bool>,
    set_toast_message: WriteSignal<ToastMessage>,
    set_refetch_trigger: WriteSignal<()>,
) -> impl IntoView {
    let name = RwSignal::new(String::new());
    let title = RwSignal::new(String::new());
    let level = RwSignal::new(String::new());
    let comp = RwSignal::new(String::new());

    let on_click = move |_| {
        if let Ok(c) = comp.get().parse::<i32>() {
            let req = AddPersonRequest::new(name.get(), title.get(), level.get(), c);
            if req.validate().is_ok() {
                spawn_local(async move {
                    if add_person(req).await.is_ok() {
                        set_refetch_trigger.set(());
                        set_if_show_modal.set(false);
                        set_toast_message.set(ToastMessage::create(ToastMessageType::NewMemberAdded));
                        set_if_show_added.set(true);
                    }
                });
            }
        }
    };

    view! {
        <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/60">
            <div class="bg-[#222222] border-t-8 border-[#7734e7] p-8 w-full max-w-md">
                <h2 class="text-white text-3xl mb-6">"Add Member"</h2>
                <input type="text" placeholder="Name" class="w-full bg-[#333333] text-white p-3 mb-4" on:input=move |e| name.set(event_target_value(&e)) />
                <input type="text" placeholder="Title" class="w-full bg-[#333333] text-white p-3 mb-4" on:input=move |e| title.set(event_target_value(&e)) />
                <input type="text" placeholder="Level" class="w-full bg-[#333333] text-white p-3 mb-4" on:input=move |e| level.set(event_target_value(&e)) />
                <input type="text" placeholder="Comp" class="w-full bg-[#333333] text-white p-3 mb-6" on:input=move |e| comp.set(event_target_value(&e)) />
                <div class="flex justify-end gap-4">
                    <button class="text-stone-400" on:click=move |_| set_if_show_modal.set(false)>"Cancel"</button>
                    <button class="bg-[#7734e7] text-white px-6 py-2 rounded" on:click=on_click>"Add"</button>
                </div>
            </div>
        </div>
    }
}
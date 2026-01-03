use crate::app::components::{ ToastMessage, ToastMessageType};
use crate::app::components::toast::ToastTrait;
use crate::app::models::AddPersonRequest;
use crate::app::server_functions::persons::add_person;
use leptos::task::spawn_local;
use leptos::prelude::*; // Use the 0.8 prelude
use validator::Validate;

#[component]
pub fn AddPersonModal(
    set_if_show_modal: WriteSignal<bool>,
    set_if_show_added: WriteSignal<bool>,
    set_toast_message: WriteSignal<ToastMessage>,
) -> impl IntoView {
    // Styles remain the same (Tailwind)
    const INPUT_STYLE: &str = "w-full h-12 bg-[#333333] pr-4 pl-6 py-4 text-white mt-6 outline-none focus:outline-none focus:pl-7 transition-all duration-1000 ease-in-out";
    const CANCEL_BUTTON_STYLE: &str = "mt-10 bg-[#555555] px-8 py-2 rounded text-white mr-3 transition-all duration-1000 ease-in-out hover:bg-[#666666]";
    const ADD_BUTTON_STYLE: &str = "mt-10 bg-[#7734e7] px-8 py-2 rounded text-white transition-all duration-1000 ease-in-out hover:bg-[#8448e9]";
    const NO_ERROR_STYLE: &str = "flex flex-col bg-[#222222] border-t-8 border-[#7734e7] px-6 pt-5 h-[29rem] w-full max-w-[36rem] z-50 -mt-2 fixed z-50";
    const ERROR_STYLE: &str = "flex flex-col bg-[#222222] border-t-8 border-[#7734e7] px-6 pt-5 h-[32rem] w-full max-w-[36rem] z-50 -mt-2 fixed z-50";

    // 0.8 Signals: use RwSignal or move to the new Signal pattern
    let person_name = RwSignal::new(String::new());
    let person_title = RwSignal::new(String::new());
    let person_level = RwSignal::new(String::new());
    let compensation = RwSignal::new(String::new());

    let error_message = RwSignal::new(String::new());
    let if_error = RwSignal::new(false);

    let on_close = move |_| {
        set_if_show_modal.set(false); // Use .set() instead of calling as a function
    };

    let on_click = move |_| {
        // Parsing with a safer fallback than .expect()
        let comp_val = compensation.get().parse::<i32>().unwrap_or(0);

        let add_person_request = AddPersonRequest::new(
            person_name.get(),
            person_title.get(),
            person_level.get(),
            comp_val,
        );

        match add_person_request.validate() {
            Ok(_) => {
                // Leptos 0.8 uses spawn_local similarly, but signals are captured differently
                spawn_local(async move {
                    match add_person(add_person_request).await {
                        Ok(_) => {
                            set_if_show_modal.set(false);
                            set_toast_message.set(ToastMessage::create(
                                ToastMessageType::NewMemberAdded,
                            ));
                            set_if_show_added.set(true);
                        }
                        Err(e) => leptos::logging::log!("Error adding: {:?}", e),
                    }
                });
            }
            Err(_) => {
                if_error.set(true);
                error_message.set("All fields are required".to_string());
            }
        }
    };

    view! {
        <div class="flex flex-col w-full h-full z-50 mx-auto items-center justify-center">
            <div class=move || if if_error.get() { ERROR_STYLE } else { NO_ERROR_STYLE }>
                <Show when=move || if_error.get()>
                    <p class="text-white bg-red-500 rounded w-full h-12 px-5 py-3 transition-all duration-750">
                        {move || error_message.get()}
                    </p>
                </Show>

                <p class="text-white pt-5">"Add New Employee"</p>

                <input type="text" placeholder="Name"
                    class=INPUT_STYLE
                    prop:value=move || person_name.get()
                    on:input=move |ev| person_name.set(event_target_value(&ev))
                />
                <input type="text" placeholder="Title"
                    class=INPUT_STYLE
                    prop:value=move || person_title.get()
                    on:input=move |ev| person_title.set(event_target_value(&ev))
                />
                <input type="text" placeholder="Level"
                    class=INPUT_STYLE
                    prop:value=move || person_level.get()
                    on:input=move |ev| person_level.set(event_target_value(&ev))
                />
                <input type="text" placeholder="Compensation"
                    class=INPUT_STYLE
                    prop:value=move || compensation.get()
                    on:input=move |ev| compensation.set(event_target_value(&ev))
                />

                <div class="flex flex-row w-full items-end justify-end">
                    <button on:click=on_close class=CANCEL_BUTTON_STYLE>
                        "Cancel"
                    </button>
                    <button on:click=on_click class=ADD_BUTTON_STYLE>
                        "Add"
                    </button>
                </div>
            </div>
        </div>
    }
}
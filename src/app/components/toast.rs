use leptos::prelude::*; // Use 0.8 prelude
use std::time::Duration;

const TOAST_PARENT_STYLE: &str = "flex flex-row top-0 h-16 w-full max-w-[61rem] \
    mx-auto items-center justify-center align-center fixed -mt-36 \
    transition-all duration-1000 ease-in-out";

const TOAST_PARENT_APPEAR_STYLE: &str = "flex flex-row top-0 h-16 w-full max-w-[61rem] \
    mx-auto items-center justify-center align-center fixed mt-20 \
    transition-all duration-1000 ease-in-out";

const TOAST_STYLE: &str = "flex w-96 h-16 bg-[#333333] rounded px-10 py-4 \
    text-white transition-all duration-1000 ease-in-out items-center";

pub enum ToastMessageType {
    NewMemberAdded,
    MemberDeleted,
    MemberUpdated,
}

pub type ToastMessage = String;

pub trait ToastTrait { // Renamed trait to avoid conflict with component name
    fn create(toast_message_type: ToastMessageType) -> ToastMessage;
}

impl ToastTrait for ToastMessage {
    fn create(toast_message_type: ToastMessageType) -> ToastMessage {
        match toast_message_type {
            ToastMessageType::NewMemberAdded => String::from("New member added"),
            ToastMessageType::MemberDeleted => String::from("Member deleted"),
            ToastMessageType::MemberUpdated => String::from("Member updated"),
        }
    }
}

#[component]
pub fn Toast(
    #[prop(into)] toast_message: Signal<ToastMessage>,
    #[prop(into)] if_appear: Signal<bool>,
    set_if_appear: WriteSignal<bool>,
) -> impl IntoView {
    
    // In 0.8, Effect replaces create_effect
    Effect::new(move |_| {
        if if_appear.get() {
            // set_timeout is still fine, or use leptos::prelude::set_timeout
            set_timeout(
                move || {
                    set_if_appear.set(false);
                },
                Duration::from_secs(4),
            );
        }
    });

    view! {
        <div class=move || {
            if if_appear.get() { TOAST_PARENT_APPEAR_STYLE }
            else { TOAST_PARENT_STYLE }
        }>
            <div class=TOAST_STYLE>
                {move || toast_message.get()}
            </div>
        </div>
    }
}
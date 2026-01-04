use leptos::prelude::*;
use std::time::Duration;

const TOAST_PARENT_STYLE: &str = "flex flex-row top-0 h-16 w-full max-w-[61rem] \
    mx-auto items-center justify-center align-center fixed -mt-36 \
    transition-all duration-1000 ease-in-out";

const TOAST_PARENT_APPEAR_STYLE: &str = "flex flex-row top-0 h-16 w-full max-w-[61rem] \
    mx-auto items-center justify-center align-center fixed mt-20 \
    transition-all duration-1000 ease-in-out";

const TOAST_STYLE: &str = "flex w-96 h-16 bg-[#333333] rounded px-10 py-4 \
    text-white transition-all duration-1000 ease-in-out items-center";

#[derive(Clone, Debug, PartialEq)]
pub enum ToastMessageType {
    NewMemberAdded,
    MemberDeleted,
    MemberUpdated,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ToastMessage {
    pub content: String,
}

impl ToastMessage {
    pub fn new() -> Self {
        Self { content: String::new() }
    }

    pub fn create(toast_message_type: ToastMessageType) -> Self {
        let msg = match toast_message_type {
            ToastMessageType::NewMemberAdded => "New member added",
            ToastMessageType::MemberDeleted => "Member deleted",
            ToastMessageType::MemberUpdated => "Member updated",
        };
        Self { content: msg.to_string() }
    }
}

#[component]
pub fn Toast(
    #[prop(into)] toast_message: Signal<ToastMessage>,
    #[prop(into)] if_appear: Signal<bool>,
    set_if_appear: WriteSignal<bool>,
) -> impl IntoView {
    
    Effect::new(move |_| {
        if if_appear.get() {
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
                {move || toast_message.get().content}
            </div>
        </div>
    }
}
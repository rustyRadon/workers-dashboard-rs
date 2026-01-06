use chrono::{Datelike, Local, Month};
use leptos::prelude::*;

#[component] 
pub fn DashboardHeader() -> impl IntoView {
    let current_now = Local::now();
    let month_number = u8::try_from(current_now.month()).unwrap();
    let current_month = Month::try_from(month_number).ok().unwrap();
    let display_date = format!("{:?}, {:?}", current_month, current_now.year());

    view! {
        <div class="flex flex-col mt-28 h-48 w-full max-w-[53rem] mx-auto items-center justify-center px-2">
            <div class="w-full flex flex-row bg-[#38000A] rounded h-full px-10 py-10">
                <div class="w-1/2 h-full flex flex-col">
                    <div class="text-white">{display_date}</div>
                    <div class="text-white text-6xl pt-2">"Team Report"</div>
                </div>
                <div class="w-1/2">
                    <img src="assets/image_1.png" class="w-[210px] -mt-40 -ml-20" />
                </div>
            </div>
        </div>
    }
}
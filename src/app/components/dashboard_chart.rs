use crate::app::components::DashboardWidget;
use crate::app::models::Person;
use charts_rs::{BarChart, Color, Series, THEME_DARK};
use leptos::prelude::*;
use num_format::{Buffer, Locale};

#[component]
pub fn DashboardChart(persons_data: Vec<Person>) -> impl IntoView {
    let team_count = persons_data.len().to_string();
    let mut total_cost: i32 = 0;
    let mut latest_member: String = String::new();
    
    let mut data_vec = Vec::new();
    let mut count_vec: Vec<f32> = Vec::new();

    for (i, person) in persons_data.into_iter().enumerate() {
        if i == 0 { latest_member = person.name.clone(); }
        total_cost += person.compensation;

        if let Some(index) = data_vec.iter().position(|title| title == &person.title) {
            count_vec[index] += 1.0;
        } else {
            data_vec.push(person.title);
            count_vec.push(1.0);
        }
    }

    let mut bar_series = Series::new(String::new(), count_vec);
    bar_series.label_show = true;

    let mut bar_chart = BarChart::new_with_theme(vec![bar_series], data_vec, THEME_DARK);
    bar_chart.width = 832.0;
    bar_chart.height = 500.0;
    bar_chart.y_axis_hidden = true;
    bar_chart.background_color = Color::transparent();

    let mut buf = Buffer::default();
    buf.write_formatted(&total_cost, &Locale::en);
    let total_cost_str = format!("${}", buf.as_str());

view! {
    <div class="w-full flex flex-col max-w-[64rem] mx-auto pt-8 mb-10">
        <div class="w-full h-20 grid grid-cols-3 gap-4 mx-auto px-2 max-w-[53rem]">
            // Pass the strings directly, not closures
            <DashboardWidget title="Team Members" value=team_count.clone() />
            <DashboardWidget title="Monthly Team Cost" value=total_cost_str.clone() />
            <DashboardWidget title="Just Joined" value=latest_member.clone() />
        </div>

        <div class="max-w-[54rem] mx-auto w-full flex flex-col mt-14 pb-12">
            <div class="w-full max-w-[41rem] h-20 bg-black-200 rounded py-10 px-4 pb-10" 
                 prop:inner_html=bar_chart.svg().unwrap()></div>
        </div>
    </div>
}}
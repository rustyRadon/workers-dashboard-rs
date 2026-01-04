use crate::app::components::DashboardWidget;
use crate::app::models::person::Person;
use charts_rs::{BarChart, Color, Series, THEME_DARK};
use leptos::prelude::*;
use num_format::{Buffer, Locale};

#[component]
pub fn DashboardChart(persons_data: Vec<Person>) -> impl IntoView {
    let team_count = persons_data.len().to_string();
    let mut total_cost: i32 = 0;
    let mut latest_member: String = "No data".to_string();
    
    let mut data_vec = Vec::new();
    let mut count_vec: Vec<f32> = Vec::new();

    for (i, person) in persons_data.iter().enumerate() {
        if i == 0 { latest_member = person.name.clone(); }
        total_cost += person.compensation;

        if let Some(index) = data_vec.iter().position(|title| title == &person.title) {
            count_vec[index] += 1.0;
        } else {
            data_vec.push(person.title.clone());
            count_vec.push(1.0);
        }
    }

    let mut bar_series = Series::new(String::new(), count_vec);
    bar_series.label_show = true;

    let mut bar_chart = BarChart::new_with_theme(vec![bar_series], data_vec, THEME_DARK);
    bar_chart.width = 832.0;
    bar_chart.height = 400.0;
    bar_chart.y_axis_hidden = true;
    bar_chart.background_color = Color::transparent();


    bar_chart.series_colors = vec![
        Color::from((205, 28, 24)),   // #CD1C18
        Color::from((155, 19, 19)),   // #9B1313
        Color::from((255, 168, 150)), // #FFA896
    ];

    let mut buf = Buffer::default();
    buf.write_formatted(&total_cost, &Locale::en);
    let total_cost_str = format!("${}", buf.as_str());

    let chart_svg = bar_chart.svg().unwrap_or_default();

    view! {
        <div class="w-full flex flex-col max-w-[64rem] mx-auto pt-8 mb-10">
            <div class="w-full h-20 grid grid-cols-3 gap-4 mx-auto px-2 max-w-[53rem]">
                <DashboardWidget title="Team Members" value=team_count />
                <DashboardWidget title="Monthly Team Cost" value=total_cost_str />
                <DashboardWidget title="Just Joined" value=latest_member />
            </div>

            <div class="max-w-[54rem] mx-auto w-full flex flex-col mt-14 pb-12">
                <div 
                    class="w-full max-w-[52rem] min-h-[420px] bg-[#38000A] rounded-xl py-6 px-4 shadow-2xl border border-[#CD1C18]/20" 
                    inner_html=chart_svg>
                </div>
                
                <p class="text-[#FFA896] text-sm mt-4 text-center font-medium uppercase tracking-widest">
                    "Staff Distribution by Role"
                </p>
            </div>
        </div>
    }
}
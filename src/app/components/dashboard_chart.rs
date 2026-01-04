use crate::app::components::DashboardWidget;
use crate::app::models::Person;
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

    // 1. Process Data
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

    // 2. Build Chart
    let mut bar_series = Series::new(String::new(), count_vec);
    bar_series.label_show = true;

    // Use a slightly smaller height (400) to ensure it fits well in the dashboard
    let mut bar_chart = BarChart::new_with_theme(vec![bar_series], data_vec, THEME_DARK);
    bar_chart.width = 832.0;
    bar_chart.height = 400.0; 
    bar_chart.y_axis_hidden = true;
    bar_chart.background_color = Color::transparent();

    // 3. Format Strings
    let mut buf = Buffer::default();
    buf.write_formatted(&total_cost, &Locale::en);
    let total_cost_str = format!("${}", buf.as_str());

    // Generate SVG string
    let chart_svg = bar_chart.svg().unwrap_or_default();

    view! {
        <div class="w-full flex flex-col max-w-[64rem] mx-auto pt-8 mb-10">
            // Statistics Widgets
            <div class="w-full h-20 grid grid-cols-3 gap-4 mx-auto px-2 max-w-[53rem]">
                <DashboardWidget title="Team Members" value=team_count />
                <DashboardWidget title="Monthly Team Cost" value=total_cost_str />
                <DashboardWidget title="Just Joined" value=latest_member />
            </div>

            // Chart Container
            <div class="max-w-[54rem] mx-auto w-full flex flex-col mt-14 pb-12">
                /* FIXED: 
                   - Changed h-20 to min-h-[420px] 
                   - Using inner_html directly
                   - Added a dark background so the THEME_DARK text is visible
                */
                <div 
                    class="w-full max-w-[52rem] min-h-[420px] bg-[#1e1e2e] rounded-xl py-6 px-4 shadow-2xl border border-white/5" 
                    inner_html=chart_svg>
                </div>
            </div>
        </div>
    }
}
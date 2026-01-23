use chrono::{Datelike, Duration, Local, NaiveDate};
use leptos::prelude::*;
use leptos::*;

#[component]
pub fn Events() -> impl IntoView {
    let today = Local::now().naive_local();

    let current_day = NaiveDate::from_ymd_opt(today.year(), today.month(), today.day());

    let day_from_0 = current_day.unwrap().day0();

    let start = current_day.unwrap() - Duration::days((day_from_0 + 2) as i64);

    let days = (0..42)
        .map(|i| {
            let days = start + chrono::Duration::days(i);
            let day_name = days.weekday().to_string();
            (days, day_name)
        })
        .collect::<Vec<_>>();

    view! {}
}

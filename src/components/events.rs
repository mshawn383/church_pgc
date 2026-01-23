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


        let first_seven_weekdays_name=days.iter().take(7).map(|(_,name)| name.clone()).collect::<Vec<_>>();
    view! {
     <div class="
    w-full max-w-xl mx-auto
    bg-white shadow-sm rounded-xl
    px-6 py-3
    grid grid-cols-3 items-center
">

    <button
        class="w-10 h-10 flex items-center justify-center
               rounded-full hover:bg-gray-100 text-gray-700"
    >
        <span class="text-xl">"<"</span>
    </button>


    <h2 class="text-center text-2xl font-semibold text-gray-900">
        {move || today.format("%B %Y").to_string()}
    </h2>


    <button
        class="w-10 h-10 flex items-center justify-center
               rounded-full hover:bg-gray-100 text-gray-700 justify-self-end"
    >
        <span class="text-xl">">"</span>
    </button>
</div>


<div class="w-full flex justify-center">
        <div class="w-full max-w-7xl flex flex-row gap-6 p-4">

           
            <div class="calendar-container w-full max-w-6xl mx-auto bg-white rounded-2xl border border-gray-300 p-8 shadow-xl">


    <div class="calendar-header flex items-center justify-center mb-8">
      

        <h2 class="current-month text-2xl font-bold text-gray-900">
            {move || today.format("%B %Y").to_string()}
        </h2>

    </div>

 
    <div class="calendar-grid grid grid-cols-7 gap-[1px] bg-gray-200 border border-gray-200 rounded-lg overflow-hidden">
        {first_seven_weekdays_name.into_iter().map(|label| view! {
            <div class="calendar-day-label bg-gray-50 p-3 text-center font-semibold text-[0.75rem] uppercase text-gray-500 tracking-wider">
                {label}
            </div>
        }).collect::<Vec<_>>()}
    </div>

    
    <div class="calendar-grid grid grid-cols-7 gap-[1px] bg-gray-200 border border-gray-200 rounded-lg overflow-hidden mt-1">
        <For
            each=move || days.clone().into_iter()
            key=|(day, _)| *day
            children=move |(day, day_name)| {

                let is_today = day == chrono::Local::now().date_naive();

                view! {
                    <div class=format!(
                        "calendar-cell bg-white min-h-[120px] p-3 relative transition
                         hover:bg-gray-50 {}",
                        if is_today { "bg-blue-50" } else { "" }
                    )>

                        <span class=format!(
                            "date-number block text-sm font-medium mb-2 {}",
                            if is_today { "text-blue-600 font-extrabold" } else { "text-gray-700" }
                        )>
                            {day.day()}
                        </span>

                      
                        <div class="event-pill event-worship text-[0.7rem] px-2 py-1 rounded-md mb-1 font-semibold bg-sky-100 text-sky-700 cursor-pointer">
                            "Worship Service"
                        </div>

                        <div class="event-pill event-youth text-[0.7rem] px-2 py-1 rounded-md mb-1 font-semibold bg-green-100 text-green-700 cursor-pointer">
                            "Youth Meeting"
                        </div>

                    </div>
                }
            }
        />
    </div>

</div>

            <div class="w-[500px] bg-white border border-gray-300 rounded-2xl p-6 shadow-md">


  <div class="flex items-center justify-between mb-6">
    <h2 class="font-bold text-xl text-gray-900">Upcoming Events</h2>
    <a href="#" class="text-sm text-blue-500 font-semibold hover:underline">
      View All
    </a>
  </div>

  <div class="flex gap-4 pb-5 mb-5 border-b border-gray-100">

    <div class="flex flex-col items-center justify-center min-w-14 h-14 rounded-full bg-blue-50 text-blue-500">
      <span class="text-[0.7rem] font-bold uppercase">FEB</span>
      <span class="text-lg font-extrabold">12</span>
    </div>


    <div class="flex-1">
      <h4 class="text-base font-semibold text-gray-900 mb-1">
        Sunday Worship Service
      </h4>
      <p class="text-sm text-gray-500 leading-snug mb-2">
        Join us for a powerful time of worship and fellowship.
      </p>

      <div class="flex items-center justify-between text-xs text-gray-500">
        <span>10:00 AM</span>
        <span class="text-blue-500 font-semibold cursor-pointer hover:underline">
          Details
        </span>
      </div>
    </div>
  </div>

  
  <div class="flex gap-4 pb-5 mb-5 border-b border-gray-100">
    <div class="flex flex-col items-center justify-center min-w-14 h-14 rounded-full bg-blue-50 text-blue-500">
      <span class="text-[0.7rem] font-bold uppercase">FEB</span>
      <span class="text-lg font-extrabold">14</span>
    </div>

    <div class="flex-1">
      <h4 class="text-base font-semibold text-gray-900 mb-1">
        Youth Prayer Meeting
      </h4>
      <p class="text-sm text-gray-500 leading-snug mb-2">
        A special gathering for young believers to pray together.
      </p>

      <div class="flex items-center justify-between text-xs text-gray-500">
        <span>Wednesday</span>
        <span class="text-blue-500 font-semibold cursor-pointer hover:underline">
          Details
        </span>
      </div>
    </div>
  </div>

 
  <div class="flex gap-4">
    <div class="flex flex-col items-center justify-center min-w-14 h-14 rounded-full bg-blue-50 text-blue-500">
      <span class="text-[0.7rem] font-bold uppercase">FEB</span>
      <span class="text-lg font-extrabold">16</span>
    </div>

    <div class="flex-1">
      <h4 class="text-base font-semibold text-gray-900 mb-1">
        Fasting Prayer
      </h4>
      <p class="text-sm text-gray-500 leading-snug mb-2">
        A dedicated time of fasting and seeking God s presence.
      </p>

      <div class="flex items-center justify-between text-xs text-gray-500">
        <span>Friday</span>
        <span class="text-blue-500 font-semibold cursor-pointer hover:underline">
          Details
        </span>
      </div>
    </div>
  </div>

</div>

        </div>
    </div>





    }
}

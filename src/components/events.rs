use chrono::{Datelike, Duration, Local, NaiveDate};
use leptos::prelude::*;
use leptos::*;

#[component]
pub fn Events() -> impl IntoView {
    let today = Local::now().naive_local();

    let (current_day, set_current_day) = signal(NaiveDate::from_ymd_opt(today.year(), today.month(), today.day()));

    let day_from_0 = current_day.get().unwrap().day0();

    let start = current_day.get().unwrap() - Duration::days((day_from_0 + 2) as i64);

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
    w-full max-w-2xl mx-auto
    bg-white border border-gray-200 rounded-lg
    px-6 py-4
    grid grid-cols-3 items-center
    mb-6
">

    <button
        class="w-10 h-10 flex items-center justify-center
               rounded-lg hover:bg-gray-100 text-gray-700 font-light text-lg"
    >
        <span>"<"</span>
    </button>


    <h2 class="text-center text-xl font-light text-gray-900">
        {move || today.format("%B %Y").to_string()}
    </h2>


    <button
        class="w-10 h-10 flex items-center justify-center
               rounded-lg hover:bg-gray-100 text-gray-700 font-light text-lg justify-self-end"
    >
        <span>">"</span>
    </button>
</div>


<div class="w-full flex justify-center">
        <div class="w-full max-w-7xl flex flex-row gap-6 p-4">

           
            <div class="calendar-container w-full max-w-6xl mx-auto bg-white rounded-lg border border-gray-200 p-6 shadow-md">


    <div class="calendar-header flex items-center justify-center mb-6">
      

        <h2 class="current-month text-2xl font-light text-gray-900">
            {move || today.format("%B %Y").to_string()}
        </h2>

    </div>

 
    <div class="calendar-grid grid grid-cols-7 gap-[1px] bg-gray-200 border border-gray-200 rounded-t-lg overflow-hidden">
        {first_seven_weekdays_name.into_iter().map(|label| view! {
            <div class="calendar-day-label bg-gray-50 p-3 text-center font-medium text-[0.7rem] uppercase text-gray-500 tracking-wide">
                {label}
            </div>
        }).collect::<Vec<_>>()}
    </div>

    
    <div class="calendar-grid grid grid-cols-7 gap-[1px] bg-gray-200 border border-gray-200 rounded-b-lg overflow-hidden">
        <For
            each=move || days.clone().into_iter()
            key=|(day, _)| *day
            children=move |(day, day_name)| {

                let is_today = day == chrono::Local::now().date_naive();

                view! {
                    <div class=format!(
                        "calendar-cell bg-white min-h-[100px] p-2 relative transition hover:bg-gray-50 {}",
                        if is_today { "bg-blue-50 border-2 border-[#4285F4]" } else { "" }
                    )>

                        <span class=format!(
                            "date-number block text-sm font-medium mb-2 {}",
                            if is_today { "text-[#4285F4] font-semibold" } else { "text-gray-700" }
                        )>
                            {day.day()}
                        </span>

                      
                        <div class="event-pill event-worship text-[0.65rem] px-2 py-0.5 rounded text-white bg-[#4285F4] cursor-pointer mb-1">
                            "Worship"
                        </div>

                        <div class="event-pill event-youth text-[0.65rem] px-2 py-0.5 rounded text-white bg-[#34A853] cursor-pointer">
                            "Youth"
                        </div>

                    </div>
                }
            }
        />
    </div>

</div>

            <div class="w-[500px] bg-white border border-gray-200 rounded-lg p-6 shadow-md">


  <div class="flex items-center justify-between mb-6 border-b border-gray-100 pb-4">
    <h2 class="font-light text-xl text-gray-900">Upcoming Events</h2>
    <a href="#" class="text-sm text-[#4285F4] font-medium hover:underline">
      View All
    </a>
  </div>

  <div class="flex gap-4 pb-4 mb-4 border-b border-gray-100">

    <div class="flex flex-col items-center justify-center min-w-12 h-12 rounded-lg bg-[#4285F4] text-white text-center">
      <span class="text-[0.6rem] font-medium uppercase leading-tight">FEB</span>
      <span class="text-base font-semibold">12</span>
    </div>


    <div class="flex-1">
      <h4 class="text-base font-medium text-gray-900 mb-1">
        Sunday Worship Service
      </h4>
      <p class="text-xs text-gray-600 leading-snug mb-2">
        Join us for a powerful time of worship and fellowship.
      </p>

      <div class="flex items-center justify-between text-xs text-gray-500">
        <span>10:00 AM</span>
        <span class="text-[#4285F4] font-medium cursor-pointer hover:underline">
          Details
        </span>
      </div>
    </div>
  </div>

  
  <div class="flex gap-4 pb-4 mb-4 border-b border-gray-100">
    <div class="flex flex-col items-center justify-center min-w-12 h-12 rounded-lg bg-[#34A853] text-white text-center">
      <span class="text-[0.6rem] font-medium uppercase leading-tight">FEB</span>
      <span class="text-base font-semibold">14</span>
    </div>

    <div class="flex-1">
      <h4 class="text-base font-medium text-gray-900 mb-1">
        Youth Prayer Meeting
      </h4>
      <p class="text-xs text-gray-600 leading-snug mb-2">
        A special gathering for young believers to pray together.
      </p>

      <div class="flex items-center justify-between text-xs text-gray-500">
        <span>Wednesday</span>
        <span class="text-[#4285F4] font-medium cursor-pointer hover:underline">
          Details
        </span>
      </div>
    </div>
  </div>

 
  <div class="flex gap-4">
    <div class="flex flex-col items-center justify-center min-w-12 h-12 rounded-lg bg-[#FBBC04] text-white text-center">
      <span class="text-[0.6rem] font-medium uppercase leading-tight">FEB</span>
      <span class="text-base font-semibold">16</span>
    </div>

    <div class="flex-1">
      <h4 class="text-base font-medium text-gray-900 mb-1">
        Fasting Prayer
      </h4>
      <p class="text-xs text-gray-600 leading-snug mb-2">
        A dedicated time of fasting and seeking God s presence.
      </p>

      <div class="flex items-center justify-between text-xs text-gray-500">
        <span>Friday</span>
        <span class="text-[#4285F4] font-medium cursor-pointer hover:underline">
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

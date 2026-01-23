use leptos::prelude::*;

#[component]
pub fn DailyQuotes() -> impl IntoView {
    view! {<div class="h-[95vh] bg-gray-100 text-black p-6 bg-[url('/assets/landscape.jpg')]">

      <h1 class="text-4xl font-bold text-center mb-8 tracking-wide">
        Daily Quotes
      </h1>

    <div class="bg-[#5aadd4] backdrop-blur-lg rounded-2xl shadow-2xl p-14 max-w-4xl mx-auto text-center mb-16">
      <p class="text-4xl font-bold leading-relaxed text-white">
        The Lord is my shepherd; I shall not want.
      </p>
      <p class="text-white mt-6 text-lg">
        Psalm 23:1
      </p>
    </div>

      <div class="w-full h-1 bg-black/30 mb-8"></div>


    <div class="max-w-6xl mx-auto">
      <h2 class="text-2xl font-semibold mb-6 text-center text-gray-800">Previous Quotes</h2>

      <div class="flex gap-6  pb-4">
        <div class="bg-white shadow-md rounded-lg p-6 border border-gray-200 min-w-[280px]">
          <p class="text-lg text-gray-900">Be the change you wish to see in the world.</p>
          <p class="text-sm text-gray-600 mt-2"> Mahatma Gandhi (Nov 16, 2023)</p>
        </div>

        <div class="bg-white shadow-md rounded-lg p-6 border border-gray-200 min-w-[280px]">
          <p class="text-lg text-gray-900">In three words I can sum up everything I ve learned about life: it goes on.</p>
          <p class="text-sm text-gray-600 mt-2"> Robert Frost (Nov 15, 2023)</p>
        </div>

        <div class="bg-white shadow-md rounded-lg p-6 border border-gray-200 min-w-[280px]">
          <p class="text-lg text-gray-900">The only way to do great work is to love what you do.</p>
          <p class="text-sm text-gray-600 mt-2"> Steve Jobs (Nov 14, 2023)</p>
        </div>

        <div class="bg-white shadow-md rounded-lg p-6 border border-gray-200 min-w-[280px]">
          <p class="text-lg text-gray-900">Life is what happens when you re busy making other plans.</p>
          <p class="text-sm text-gray-600 mt-2"> John Lennon (Nov 13, 2023)</p>
        </div>
      </div>
    </div>
    </div>
        }
}

use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_navigate};

#[component]
pub fn Navigation() -> impl IntoView {
    let navigate = use_navigate();
    view! {
        <nav class="w-full bg-white border-b border-gray-200 px-6 py-4 shadow-sm">
          <div class="max-w-7xl mx-auto flex items-center">

            <div class="flex-1">
              <span class="text-2xl font-light text-[#4285F4] cursor-pointer font-extrabold" on:click=move |_| navigate("/",Default::default())>PGC</span>
            </div>


            <div class="flex-1 hidden md:flex items-center justify-center space-x-8">
              <A href="/daily-quotes" attr:class="text-gray-700 font-medium hover:text-[#4285F4] transition">Daily Quotes</A>

              <button
                class="px-4 py-2 rounded text-white bg-[#EA4335] hover:bg-[#D33425] transition font-medium text-sm" >
                YouTube
              </button>


              <A href="/gallery" attr:class="text-gray-700 font-medium hover:text-[#4285F4] transition">Gallery</A>
            </div>


            <div class="flex-1 flex justify-end">
              <a href="#" class="text-gray-700 font-medium hover:text-[#4285F4] transition">Login</a>
            </div>

          </div>
        </nav>
    }
}

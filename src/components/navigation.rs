use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_navigate};

#[component]
pub fn Navigation() -> impl IntoView {
    let navigate = use_navigate();
   
 
    view! {
        <nav class="w-full bg-white border-b border-gray-200 px-6 py-4 shadow-sm">
          <div class="max-w-7xl mx-auto flex items-center">

            <div class="flex-1">
              <span class="text-2xl font-extrabold text-amber-600 cursor-pointer" on:click=move |_| navigate.clone()("/",Default::default())>PGC</span>
            </div>


            <div class="flex-1 hidden md:flex items-center justify-center space-x-8">
              <A href="/daily-quotes" attr:class="text-gray-700 font-medium hover:text-amber-600 transition">Daily Quotes</A>

              <A href="https://www.youtube.com/@2025prophetAmrit" attr:class="px-4 py-2 rounded text-white bg-rose-600 hover:bg-rose-700 transition font-medium text-sm"  target="_blank">
                YouTube
              </A>


              <A href="/gallery" attr:class="text-gray-700 font-medium hover:text-amber-700 transition">Gallery</A>
            </div>


            <div class="flex-1 flex justify-end">
              <A href="/login" attr:class="px-4 py-2 rounded font-medium text-white bg-amber-600 hover:bg-amber-700 transition">Login</A>
            </div>

          </div>
        </nav>
    }
}

use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_navigate};

#[component]
pub fn Navigation() -> impl IntoView {
    let navigate = use_navigate();
    view! {
        <nav class="w-full bg-white  px-4 py-3">
          <div class=" mx-auto flex items-center">

            <div class="flex-1">
              <span class="text-4xl font-extrabold church-header-and-prophet-name " on:click=move |_| navigate("/",Default::default())>PGC</span>
            </div>


            <div class="flex-1 hidden md:flex items-center justify-center space-x-6">
              <A href="/daily-quotes" >Daily Quotes</A>

              <button
                class="px-4 py-2 rounded-lg text-white bg-red-600 hover:bg-red-700 transition font-medium" >
                Watch us on Youtube
              </button>


              <a href="#" class="text-gray-700 hover:text-black">Gallery</a>
            </div>


            <div class="flex-1 flex justify-end">
              <a href="#" class="text-gray-700 hover:text-black font-medium">Login</a>
            </div>

          </div>
        </nav>
    }
}

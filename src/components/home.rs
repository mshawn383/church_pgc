use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_navigate};
#[component]
pub fn Home() -> impl IntoView {
    view! {    

    <section
          class="relative w-full h-[95vh] bg-white flex flex-col overflow-hidden"
          style="background: url('https://images.pexels.com/photos/938141/pexels-photo-938141.jpeg') no-repeat top; background-size: 100% 60%;"
        >

         <div class="h-1/2"></div>

         <div class="h-1/2 flex flex-col items-center justify-center w-full">
      <div class="text-center mb-8">
        <h1 class="text-4xl md:text-5xl font-light text-gray-900 leading-tight mb-3">
          PGC Church
        </h1>

        <p class="text-base md:text-lg text-gray-600 font-light max-w-md mx-auto">
          Join us for faith, community, and spiritual growth
        </p>
      </div>

     <div class="flex flex-col gap-3 sm:flex-row sm:gap-4">

      <A
        href="/events"
        attr:class="px-8 py-2.5 bg-[#4285F4] text-white font-medium rounded-lg text-sm hover:bg-[#3367D6] transition"
      >
        Upcoming Events
      </A>

      <a
        href="#"
        class="px-8 py-2.5 border border-[#4285F4] text-[#4285F4] font-medium rounded-lg text-sm hover:bg-blue-50 transition"
      >
        Prayer Request
      </a>

    </div>
    </div>

        </section>
     
      }
}

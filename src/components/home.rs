use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_navigate};
#[component]
pub fn Home() -> impl IntoView {
    view! {    <section
          class="relative w-full h-[95vh] bg-gradient-to-br from-blue-50 via-white to-purple-50 flex flex-col items-center justify-center"
        >

         <div class="text-center mb-12">
      <div>
        <p class="text-sm md:text-base tracking-widest text-[#4285F4] font-semibold uppercase mb-2">
          Welcome to
        </p>

        <h1 class="text-5xl md:text-7xl font-light text-gray-900 leading-tight">
          PGC Church
        </h1>

        <p class="text-lg md:text-xl text-gray-600 mt-4 font-light">
          Join us for faith, community, and spiritual growth
        </p>
      </div>
    </div>

     <div class="flex flex-col gap-4">

      <A
        href="/events"
        attr:class="inline-block px-8 py-3 bg-[#4285F4] text-white font-medium rounded-lg text-base hover:bg-[#3367D6] transition shadow-md"
      >
        Upcoming Events
      </A>

      <a
        href="#"
        class="inline-block px-8 py-3 border-2 border-[#4285F4] text-[#4285F4] font-medium rounded-lg text-base hover:bg-blue-50 transition"
      >
        Prayer Request
      </a>

    </div>

        </section>}
}

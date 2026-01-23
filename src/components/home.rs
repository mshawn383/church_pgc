use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_navigate};
#[component]
pub fn Home() -> impl IntoView {
    view! {    <section
          class="relative w-full h-[95vh] bg-gray-100 bg-cover bg-center" style="background-image: url('https://images.pexels.com/photos/35760694/pexels-photo-35760694.jpeg');"
        >

         <div class="
  absolute right-6 bottom-6
  space-y-1
  text-left

  /* Mobile adjustments */
  max-w-[80%]
  [@media(max-width:510px)]:right-4
  [@media(max-width:510px)]:bottom-4
">
      <div>
        <h3 class="
      text-xs md:text-2xl
      tracking-widest text-yellow-700 font-extrabold
    ">
          PROPHET
        </h3>

        <h1 class="
      text-3xl md:text-8xl
      church-header-and-prophet-name
      text-gray-900 mt-1
      leading-tight
      [@media(max-width:510px)]:text-4xl
    ">
          AMRIT AIND
        </h1>
      </div>
    </div>
     <div class="flex flex-col gap-3 p-6
            absolute bottom-5 left-6
            md:bottom-20 md:left-6 ">

      <A
        href="/events"
        attr:class="inline-block px-6 py-3 border border-white/70 text-black font-extrabold
           rounded-sm backdrop-blur-sm bg-white/20 hover:bg-white/30 
           transition-all duration-300 shadow-md text-lg"
      >
        Upcoming Events
      </A>

      <a
        href="#"
        class="inline-block px-6 py-3 border border-white/70 text-black font-extrabold
           rounded-sm backdrop-blur-sm bg-white/20 hover:bg-white/30 
           transition-all duration-300 shadow-md text-lg"
      >
        Prayer Request
      </a>

    </div>

        </section>}
}

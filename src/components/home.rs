use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_navigate};
#[component]
pub fn Home() -> impl IntoView {
    view! {    



    <section
          class="relative w-full h-[95vh] bg-gradient-to-br from-white via-blue-50 to-white flex flex-col overflow-hidden"
        >

         <div class="absolute inset-0 flex items-center justify-center opacity-5">
          <div class="text-9xl font-light text-gray-400">"✦"</div>
         </div>

         <div class="relative h-full flex flex-col items-center justify-center w-full">
      
      <div class="mb-12 px-4 w-full">
        <div class="relative w-full h-64 md:h-96 rounded-3xl overflow-hidden shadow-2xl border-4 border-white">
          <img 
            src="https://images.pexels.com/photos/3942683/pexels-photo-3942683.jpeg?auto=compress&cs=tinysrgb&w=600"
            alt="PGC Church Community"
            class="w-full h-full object-cover"
          />
          <div class="absolute inset-0" style="background: linear-gradient(135deg, rgba(66, 133, 244, 0.1) 0%, rgba(234, 67, 53, 0.05) 100%);"></div>
        </div>
      </div>

      <div class="text-center mb-12 float-animation">
        <h1 class="text-5xl md:text-7xl font-light text-gray-900 leading-tight mb-2 tracking-tight text-glow">
          PGC Church
        </h1>
        <div class="flex justify-center gap-1 mt-4">
          <div class="w-1.5 h-1.5 rounded-full" style="background-color: #4285F4;"></div>
          <div class="w-1.5 h-1.5 rounded-full" style="background-color: #EA4335;"></div>
          <div class="w-1.5 h-1.5 rounded-full" style="background-color: #FBBC04;"></div>
        </div>
      </div>

        <div class="max-w-2xl mx-auto text-center float-animation-delayed mb-16">
          <p class="text-xl md:text-2xl text-gray-700 font-light leading-relaxed">
            Experience <span class="font-semibold" style="color: #4285F4;">faith</span>, 
            <span class="font-semibold" style="color: #EA4335;">community</span>, and 
            <span class="font-semibold" style="color: #34A853;">spiritual growth</span>
          </p>
        </div>

     <div class="flex flex-col gap-4 sm:flex-row sm:gap-6">

      <A
        href="/events"
        attr:class="px-10 py-3 bg-[#4285F4] text-white font-medium rounded-full text-base hover:shadow-lg hover:scale-105 transition-all duration-300"
      >
        Upcoming Events
      </A>

      <a
        href="#"
        class="px-10 py-3 border-2 border-[#EA4335] text-[#EA4335] font-medium rounded-full text-base hover:bg-red-50 hover:scale-105 transition-all duration-300"
      >
        Send Prayer Request
      </a>

    </div>
    </div>

        </section>
     
      }
}

use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_navigate};
#[component]
pub fn Home() -> impl IntoView {
    view! {



    <section
          class="relative w-full h-[95vh] bg-gradient-to-br from-white via-blue-50 to-white flex flex-col overflow-hidden"
        >

         <div class="relative h-[80vh] flex flex-col items-stretch justify-between w-full">

      {/* Image Section - Left Half */}
      <div class="w-full overflow-hidden">
        <div class="relative w-full h-full rounded-r-3xl overflow-hidden shadow-2xl border-4 border-white">
          <img
            src="https://images.pexels.com/photos/3942683/pexels-photo-3942683.jpeg?auto=compress&cs=tinysrgb&w=600"
            alt="PGC Church Community"
            class="w-full h-full object-cover"
          />
          <div class="absolute inset-0" style="background: linear-gradient(135deg, rgba(66, 133, 244, 0.1) 0%, rgba(234, 67, 53, 0.05) 100%);"></div>
        </div>
      </div>

      {/* Content Section - Right Half */}
      <div class="w-full flex flex-col items-center justify-center px-12">

        <div class="text-center mb-8">
          <div class="flex justify-center gap-1 mt-4">
            <div class="w-1.5 h-1.5 rounded-full" style="background-color: #4285F4;"></div>
            <div class="w-1.5 h-1.5 rounded-full" style="background-color: #EA4335;"></div>
            <div class="w-1.5 h-1.5 rounded-full" style="background-color: #FBBC04;"></div>
          </div>
        </div>

        <div class="w-3xl text-center mb-12">
          <p class="text-4xl md:text-3xl text-gray-700 font-light leading-relaxed">
           I am the way the truth and the life.
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
    </div>

        </section>

      }
}

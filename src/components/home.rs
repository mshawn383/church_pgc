use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_navigate};
#[component]
pub fn Home() -> impl IntoView {
    view! {    



    <section
          class="relative w-full min-h-screen bg-gradient-to-br from-amber-50 via-white to-rose-50 flex flex-col overflow-hidden"
        >

         // Animated background elements
         <div class="absolute inset-0 overflow-hidden pointer-events-none">
           <div class="absolute top-20 left-10 text-6xl opacity-10 text-amber-600 animate-pulse">"✝"</div>
           <div class="absolute top-40 right-20 text-5xl opacity-10 text-rose-500 animate-pulse" style="animation-delay: 1s;">"✦"</div>
           <div class="absolute bottom-40 left-20 text-7xl opacity-10 text-amber-500 animate-pulse" style="animation-delay: 2s;">"✝"</div>
           <div class="absolute bottom-20 right-40 text-5xl opacity-10 text-rose-400 animate-pulse" style="animation-delay: 1.5s;">"✦"</div>
         </div>

         // Decorative gradient orbs
         <div class="absolute top-0 left-0 w-96 h-96 bg-amber-200 rounded-full mix-blend-multiply filter blur-3xl opacity-20 animate-blob"></div>
         <div class="absolute top-0 right-0 w-96 h-96 bg-rose-200 rounded-full mix-blend-multiply filter blur-3xl opacity-20 animate-blob animation-delay-2000"></div>
         <div class="absolute bottom-0 left-1/2 w-96 h-96 bg-orange-200 rounded-full mix-blend-multiply filter blur-3xl opacity-20 animate-blob animation-delay-4000"></div>

         <div class="relative h-full flex flex-col items-center justify-center w-full py-12 px-4">
       
       // Hero Image with enhanced styling - BIGGER SIZE
       <div class="mb-12 w-full max-w-[90rem]">
         <div class="relative w-full h-96 md:h-[40rem] lg:h-[45rem] rounded-3xl overflow-hidden shadow-2xl border-4 border-white transform hover:scale-[1.02] transition-transform duration-500">
           <img 
             src="https://images.pexels.com/photos/3942683/pexels-photo-3942683.jpeg?auto=compress&cs=tinysrgb&w=600"
             alt="PGC Church Community"
             class="w-full h-full object-cover"
           />
           <div class="absolute inset-0 bg-gradient-to-t from-amber-900/50 via-transparent to-transparent"></div>
           
           // Floating badge on image
           <div class="absolute bottom-8 left-8 bg-white/95 backdrop-blur-sm px-8 py-4 rounded-full shadow-lg flex items-center gap-3">
             <span class="text-3xl">"🕊️"</span>
             <span class="text-gray-800 font-semibold text-lg">"Welcome Home"</span>
           </div>
         </div>
       </div>

       // Main heading with enhanced styling
       <div class="text-center mb-8">
         <div class="inline-block mb-4 px-6 py-2 bg-gradient-to-r from-amber-100 to-rose-100 rounded-full">
           <p class="text-sm font-semibold text-amber-800 tracking-wide uppercase">"A Place of Worship & Fellowship"</p>
         </div>
         
         <h1 class="text-6xl md:text-8xl font-bold leading-tight mb-4 tracking-tight" 
             style="background: linear-gradient(135deg, #D97706 0%, #DC2626 50%, #B91C1C 100%); -webkit-background-clip: text; -webkit-text-fill-color: transparent; background-clip: text;">
           "PGC Church"
         </h1>
         
         <div class="flex justify-center gap-2 mt-6">
           <div class="w-2 h-2 rounded-full animate-pulse" style="background-color: #D97706;"></div>
           <div class="w-2 h-2 rounded-full animate-pulse" style="background-color: #DC2626; animation-delay: 0.2s;"></div>
           <div class="w-2 h-2 rounded-full animate-pulse" style="background-color: #B91C1C; animation-delay: 0.4s;"></div>
           <div class="w-2 h-2 rounded-full animate-pulse" style="background-color: #92400E; animation-delay: 0.6s;"></div>
         </div>
       </div>

       // Tagline with enhanced typography
       <div class="max-w-4xl mx-auto text-center mb-12">
         <p class="text-3xl md:text-5xl text-gray-700 font-light leading-relaxed mb-6">
           "Experience " <span class="font-bold text-transparent bg-clip-text bg-gradient-to-r from-amber-700 to-amber-500">"faith"</span>", "
           <span class="font-bold text-transparent bg-clip-text bg-gradient-to-r from-rose-700 to-rose-500">"community"</span>", and "
           <span class="font-bold text-transparent bg-clip-text bg-gradient-to-r from-red-700 to-red-500">"spiritual growth"</span>
         </p>
         <p class="text-lg md:text-xl text-gray-600 font-light italic max-w-2xl mx-auto">
           "\"For where two or three gather in my name, there am I with them.\" - Matthew 18:20"
         </p>
       </div>

       // Feature cards
      //  <div class="grid grid-cols-1 md:grid-cols-3 gap-6 max-w-5xl mx-auto mb-12 w-full">
         
      //    <div class="bg-white/80 backdrop-blur-sm rounded-2xl p-6 shadow-lg hover:shadow-xl transition-all duration-300 hover:-translate-y-2 border border-amber-100">
      //      <div class="text-4xl mb-4">"🙏"</div>
      //      <h3 class="text-xl font-bold text-gray-800 mb-2">"Weekly Worship"</h3>
      //      <p class="text-gray-600">"Join us every Sunday for uplifting worship and powerful messages"</p>
      //    </div>

      //    <div class="bg-white/80 backdrop-blur-sm rounded-2xl p-6 shadow-lg hover:shadow-xl transition-all duration-300 hover:-translate-y-2 border border-rose-100">
      //      <div class="text-4xl mb-4">"❤️"</div>
      //      <h3 class="text-xl font-bold text-gray-800 mb-2">"Community Care"</h3>
      //      <p class="text-gray-600">"Experience genuine fellowship and support in our loving community"</p>
      //    </div>

      //    <div class="bg-white/80 backdrop-blur-sm rounded-2xl p-6 shadow-lg hover:shadow-xl transition-all duration-300 hover:-translate-y-2 border border-orange-100">
      //      <div class="text-4xl mb-4">"📖"</div>
      //      <h3 class="text-xl font-bold text-gray-800 mb-2">"Bible Study"</h3>
      //      <p class="text-gray-600">"Grow deeper in faith through engaging Bible studies and prayer"</p>
      //    </div>

      //  </div>

       // Call-to-action buttons with enhanced styling
       <div class="flex flex-col gap-4 sm:flex-row sm:gap-6 mb-8">

         <A
           href="/events"
           attr:class="group px-12 py-4 bg-gradient-to-r from-amber-600 to-amber-700 text-white font-semibold rounded-full text-lg hover:shadow-2xl hover:scale-105 transition-all duration-300 flex items-center gap-2"
         >
           <span>"Upcoming Events"</span>
           <span class="group-hover:translate-x-1 transition-transform">"→"</span>
         </A>

         <a
           href="#"
           class="group px-12 py-4 bg-white border-2 border-rose-600 text-rose-700 font-semibold rounded-full text-lg hover:bg-rose-50 hover:shadow-xl hover:scale-105 transition-all duration-300 flex items-center gap-2"
         >
           <span>"🙏"</span>
           <span>"Send Prayer Request"</span>
         </a>

       </div>

       // Additional info section
       <div class="mt-8 text-center">
         <p class="text-gray-600 mb-4">"Service Times: Sunday 9:00 AM & 11:00 AM"</p>
         <div class="flex justify-center gap-6 text-sm text-gray-500">
           <span class="flex items-center gap-2">
             <span>"📍"</span>
             <span>"Find Us"</span>
           </span>
           <span class="flex items-center gap-2">
             <span>"📞"</span>
             <span>"Contact"</span>
           </span>
           <span class="flex items-center gap-2">
             <span>"✉️"</span>
             <span>"Connect"</span>
           </span>
         </div>
       </div>

     </div>

        </section>
     
      }
}


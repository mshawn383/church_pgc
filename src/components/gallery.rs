use leptos::prelude::*;

pub fn Gallery() -> impl IntoView {
    view! {
          <div class="min-h-screen bg-white text-gray-900 font-sans px-8 py-12">

      <h1 class="text-3xl font-light mb-8">Gallery</h1>

      <div class="flex gap-2 mb-10 overflow-x-auto pb-4">
        <button class="px-6 py-2 rounded-full bg-[#4285F4] text-white text-sm font-medium whitespace-nowrap transition hover:shadow-md">
          All
        </button>

        <button class="px-6 py-2 rounded-full bg-red-100 border border-[#EA4335] text-[#EA4335] text-sm font-medium whitespace-nowrap hover:bg-red-50 transition">
          Testimony
        </button>

        <button class="px-6 py-2 rounded-full bg-yellow-100 border border-[#FBBC04] text-[#B88600] text-sm font-medium whitespace-nowrap hover:bg-yellow-50 transition">
          Shorts
        </button>

        <button class="px-6 py-2 rounded-full bg-green-100 border border-[#34A853] text-[#34A853] text-sm font-medium whitespace-nowrap hover:bg-green-50 transition">
          Sermon
        </button>
      </div>


      <div class="grid grid-cols-[repeat(auto-fill,minmax(280px,1fr))] gap-6">


        <div class="group bg-white border border-gray-200 rounded-lg overflow-hidden cursor-pointer transition-shadow duration-300 hover:shadow-lg">


          <div class="relative w-full aspect-video bg-gray-100">
            <img src="https://img.youtube.com/vi/dQw4w9WgXcQ/hqdefault.jpg"
                 class="w-full h-full object-cover" />


            <div class="absolute inset-0 flex items-center justify-center bg-black/0 group-hover:bg-black/30 transition">
              <div class="w-14 h-14 bg-white/90 rounded-full flex items-center justify-center opacity-0 transition-opacity duration-300 group-hover:opacity-100">
                <svg class="w-7 h-7 text-[#4285F4]" fill="currentColor" viewBox="0 0 24 24">
                  <path d="M8 5v14l11-7z"/>
                </svg>
              </div>
            </div>
          </div>

          <div class="p-4">
            <h3 class="text-base font-medium text-gray-900 leading-snug line-clamp-2">
              Worship Song Be Still and Know That I Am God
            </h3>

            <div class="flex justify-between items-center text-xs text-gray-600 mt-3">
              <span class="text-[#4285F4] font-medium">Hillsong Worship</span>
              <span>1.2M views</span>
            </div>
          </div>
        </div>


        <div class="group bg-white border border-gray-200 rounded-lg overflow-hidden cursor-pointer transition-shadow duration-300 hover:shadow-lg">

          <div class="relative w-full aspect-video bg-gray-100">
            <img src="https://img.youtube.com/vi/ysz5S6PUM-U/hqdefault.jpg"
                 class="w-full h-full object-cover" />

            <div class="absolute inset-0 flex items-center justify-center bg-black/0 group-hover:bg-black/30 transition">
              <div class="w-14 h-14 bg-white/90 rounded-full flex items-center justify-center opacity-0 transition-opacity duration-300 group-hover:opacity-100">
                <svg class="w-7 h-7 text-[#4285F4]" fill="currentColor" viewBox="0 0 24 24">
                  <path d="M8 5v14l11-7z"/>
                </svg>
              </div>
            </div>
          </div>

          <div class="p-4">
            <h3 class="text-base font-medium text-gray-900 leading-snug line-clamp-2">
              Youth Revival Night Full Worship Session
            </h3>

            <div class="flex justify-between items-center text-xs text-gray-600 mt-3">
              <span class="text-[#4285F4] font-medium">Revive Youth</span>
              <span>98K views</span>
            </div>
          </div>
        </div>
        Hello
      </div>
    </div>
        }
}

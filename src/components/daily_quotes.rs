use leptos::prelude::*;

#[component]
pub fn DailyQuotes() -> impl IntoView {
    view! {<div class="
    min-h-screen 
    text-gray-900 
    p-6 
    bg-white
    font-sans
">

  
  <h1 class="text-3xl font-light text-center mb-12">
    Daily Quotes
  </h1>


  <div class="
      bg-gradient-to-br from-amber-600 to-rose-600
      rounded-lg
      shadow-md
      p-12 
      max-w-4xl 
      mx-auto 
      text-center 
      mb-20
      text-white
  ">
    <p class="text-3xl font-light leading-tight mb-6">
      The Lord is my shepherd; I shall not want.
    </p>
    <p class="text-white/90 text-base italic">
      Psalm 23:1
    </p>
  </div>

 
  <div class="flex items-center justify-center my-10 px-6">
    <div class="flex-1 border-t border-gray-200"></div>
    <span class="px-6 text-xs tracking-widest uppercase text-gray-400">
      Previous Quotes
    </span>
    <div class="flex-1 border-t border-gray-200"></div>
  </div>


  <div class="max-w-7xl mx-auto grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6 px-6 pb-20">

    <div class="
        bg-white 
        rounded-lg 
        p-6 
        border-l-4 border-l-amber-600 border border-gray-200 
        flex flex-col 
        min-h-[200px]
        hover:shadow-md transition-shadow
    ">
      <p class="text-base font-medium leading-relaxed mb-4 text-gray-900">
        Be the change you wish to see in the world.
      </p>
      <p class="text-sm text-gray-600 italic">
        Mahatma Gandhi
      </p>
      <p class="text-xs text-gray-400 mt-auto pt-6">
        Nov 16, 2023
      </p>
    </div>

    <div class="
        bg-white 
        rounded-lg 
        p-6 
        border-l-4 border-l-rose-600 border border-gray-200 
        flex flex-col 
        min-h-[200px]
        hover:shadow-md transition-shadow
    ">
      <p class="text-base font-medium leading-relaxed mb-4 text-gray-900">
        In three words I can sum up everything I ve learned about life: it goes on.
      </p>
      <p class="text-sm text-gray-600 italic">
        Robert Frost
      </p>
      <p class="text-xs text-gray-400 mt-auto pt-6">
        Nov 15, 2023
      </p>
    </div>

    <div class="
        bg-white 
        rounded-lg 
        p-6 
        border-l-4 border-l-orange-600 border border-gray-200 
        flex flex-col 
        min-h-[200px]
        hover:shadow-md transition-shadow
    ">
      <p class="text-base font-medium leading-relaxed mb-4 text-gray-900">
        The only way to do great work is to love what you do.
      </p>
      <p class="text-sm text-gray-600 italic">
        Steve Jobs
      </p>
      <p class="text-xs text-gray-400 mt-auto pt-6">
        Nov 14, 2023
      </p>
    </div>

    <div class="
        bg-white 
        rounded-lg 
        p-6 
        border-l-4 border-l-amber-600 border border-gray-200 
        flex flex-col 
        min-h-[200px]
        hover:shadow-md transition-shadow
    ">
      <p class="text-base font-medium leading-relaxed mb-4 text-gray-900">
        Life is either a daring adventure or nothing at all.
      </p>
      <p class="text-sm text-gray-600 italic">
        Helen Keller
      </p>
      <p class="text-xs text-gray-400 mt-auto pt-6">
        Nov 12, 2023
      </p>
    </div>

    <div class="
        bg-white 
        rounded-lg 
        p-6 
        border-l-4 border-l-rose-600 border border-gray-200 
        flex flex-col 
        min-h-[200px]
        hover:shadow-md transition-shadow
    ">
      <p class="text-base font-medium leading-relaxed mb-4 text-gray-900">
        The purpose of our lives is to be happy.
      </p>
      <p class="text-sm text-gray-600 italic">
        Dalai Lama
      </p>
      <p class="text-xs text-gray-400 mt-auto pt-6">
        Nov 11, 2023
      </p>
    </div>

    <div class="
        bg-white 
        rounded-lg 
        p-6 
        border-l-4 border-l-orange-600 border border-gray-200 
        flex flex-col 
        min-h-[200px]
        hover:shadow-md transition-shadow
    ">
      <p class="text-base font-medium leading-relaxed mb-4 text-gray-900">
        Be yourself; everyone else is already taken.
      </p>
      <p class="text-sm text-gray-600 italic">
        Oscar Wilde
      </p>
      <p class="text-xs text-gray-400 mt-auto pt-6">
        Nov 10, 2023
      </p>
    </div>

    <div class="
        bg-white 
        rounded-lg 
        p-6 
        border-l-4 border-l-amber-700 border border-gray-200 
        flex flex-col 
        min-h-[200px]
        hover:shadow-md transition-shadow
    ">
      <p class="text-base font-medium leading-relaxed mb-4 text-gray-900">
        It is during our darkest moments that we must focus on the light.
      </p>
      <p class="text-sm text-gray-600 italic">
        Aristotle
      </p>
      <p class="text-xs text-gray-400 mt-auto pt-6">
        Nov 09, 2023
      </p>
    </div>
</div>
</div>
        }
}

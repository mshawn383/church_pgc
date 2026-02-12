use leptos::prelude::*;

#[component]
pub fn Dashboard() -> impl IntoView {
    // State for showing/hiding modals
    let (show_event_modal, set_show_event_modal) = signal(false);
    let (show_quote_modal, set_show_quote_modal) = signal(false);

    view! {
        <div class="relative w-full min-h-screen bg-gradient-to-br from-amber-50 via-white to-rose-50">
            
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

            // Main Content
            <div class="relative z-10 max-w-7xl mx-auto px-6 py-12">
                
                // Header
                <div class="mb-12">
                    <div class="flex items-center justify-between mb-6">
                        <div>
                            <h1 class="text-4xl md:text-5xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-amber-600 to-rose-600 mb-2">
                                "Admin Dashboard"
                            </h1>
                            <p class="text-gray-600 text-lg">"Manage your church content"</p>
                        </div>
                        <a href="/" class="px-6 py-3 bg-white border-2 border-amber-600 text-amber-600 font-semibold rounded-xl hover:bg-amber-50 transition-all">
                            "← Back to Home"
                        </a>
                    </div>
                    
                    // Stats Cards
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mt-8">
                        <div class="bg-white/80 backdrop-blur-sm rounded-2xl p-6 shadow-lg border border-amber-100">
                            <div class="flex items-center justify-between">
                                <div>
                                    <p class="text-gray-600 text-sm font-medium">"Total Events"</p>
                                    <p class="text-3xl font-bold text-amber-600 mt-2">"12"</p>
                                </div>
                                <div class="text-4xl">"📅"</div>
                            </div>
                        </div>
                        
                        <div class="bg-white/80 backdrop-blur-sm rounded-2xl p-6 shadow-lg border border-rose-100">
                            <div class="flex items-center justify-between">
                                <div>
                                    <p class="text-gray-600 text-sm font-medium">"Daily Quotes"</p>
                                    <p class="text-3xl font-bold text-rose-600 mt-2">"45"</p>
                                </div>
                                <div class="text-4xl">"📖"</div>
                            </div>
                        </div>
                        
                        <div class="bg-white/80 backdrop-blur-sm rounded-2xl p-6 shadow-lg border border-orange-100">
                            <div class="flex items-center justify-between">
                                <div>
                                    <p class="text-gray-600 text-sm font-medium">"Gallery Items"</p>
                                    <p class="text-3xl font-bold text-orange-600 mt-2">"28"</p>
                                </div>
                                <div class="text-4xl">"🖼️"</div>
                            </div>
                        </div>
                    </div>
                </div>

                // Quick Actions
                <div class="mb-12">
                    <h2 class="text-2xl font-bold text-gray-800 mb-6">"Quick Actions"</h2>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        
                        // Add Event Card
                        <div class="bg-white/90 backdrop-blur-sm rounded-2xl p-8 shadow-lg border border-white/50 hover:shadow-xl transition-all">
                            <div class="text-5xl mb-4">"📅"</div>
                            <h3 class="text-2xl font-bold text-gray-800 mb-3">"Add New Event"</h3>
                            <p class="text-gray-600 mb-6">"Create and schedule upcoming church events, services, and activities"</p>
                            <button 
                                on:click=move |_| set_show_event_modal.set(true)
                                class="w-full py-3 bg-gradient-to-r from-amber-600 to-amber-700 text-white font-semibold rounded-xl hover:shadow-lg hover:scale-[1.02] transition-all duration-300"
                            >
                                "Create Event"
                            </button>
                        </div>

                        // Add Quote Card
                        <div class="bg-white/90 backdrop-blur-sm rounded-2xl p-8 shadow-lg border border-white/50 hover:shadow-xl transition-all">
                            <div class="text-5xl mb-4">"📖"</div>
                            <h3 class="text-2xl font-bold text-gray-800 mb-3">"Add Daily Quote"</h3>
                            <p class="text-gray-600 mb-6">"Share inspirational Bible verses and spiritual messages with your community"</p>
                            <button 
                                on:click=move |_| set_show_quote_modal.set(true)
                                class="w-full py-3 bg-gradient-to-r from-rose-600 to-rose-700 text-white font-semibold rounded-xl hover:shadow-lg hover:scale-[1.02] transition-all duration-300"
                            >
                                "Add Quote"
                            </button>
                        </div>

                    </div>
                </div>

                // Recent Activity
                <div class="mb-12">
                    <h2 class="text-2xl font-bold text-gray-800 mb-6">"Recent Activity"</h2>
                    <div class="bg-white/90 backdrop-blur-sm rounded-2xl shadow-lg border border-white/50 overflow-hidden">
                        
                        // Activity Item
                        <div class="p-6 border-b border-gray-100 hover:bg-gray-50 transition-colors">
                            <div class="flex items-start gap-4">
                                <div class="text-3xl">"📅"</div>
                                <div class="flex-1">
                                    <h4 class="font-semibold text-gray-800">"Sunday Service Added"</h4>
                                    <p class="text-gray-600 text-sm mt-1">"New event scheduled for December 24, 2024"</p>
                                    <p class="text-gray-400 text-xs mt-2">"2 hours ago"</p>
                                </div>
                            </div>
                        </div>

                        <div class="p-6 border-b border-gray-100 hover:bg-gray-50 transition-colors">
                            <div class="flex items-start gap-4">
                                <div class="text-3xl">"📖"</div>
                                <div class="flex-1">
                                    <h4 class="font-semibold text-gray-800">"Daily Quote Published"</h4>
                                    <p class="text-gray-600 text-sm mt-1">"\"The Lord is my shepherd\" - Psalm 23:1"</p>
                                    <p class="text-gray-400 text-xs mt-2">"5 hours ago"</p>
                                </div>
                            </div>
                        </div>

                        <div class="p-6 hover:bg-gray-50 transition-colors">
                            <div class="flex items-start gap-4">
                                <div class="text-3xl">"🖼️"</div>
                                <div class="flex-1">
                                    <h4 class="font-semibold text-gray-800">"Gallery Updated"</h4>
                                    <p class="text-gray-600 text-sm mt-1">"5 new photos added to Christmas celebration album"</p>
                                    <p class="text-gray-400 text-xs mt-2">"1 day ago"</p>
                                </div>
                            </div>
                        </div>

                    </div>
                </div>

            </div>

            // Event Modal
            {move || show_event_modal.get().then(|| view! {
                <div class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-6">
                    <div class="bg-white rounded-3xl shadow-2xl max-w-2xl w-full max-h-[90vh] overflow-y-auto">
                        
                        // Modal Header
                        <div class="bg-gradient-to-r from-amber-600 to-amber-700 px-8 py-6 flex items-center justify-between">
                            <h2 class="text-2xl font-bold text-white">"Create New Event"</h2>
                            <button 
                                on:click=move |_| set_show_event_modal.set(false)
                                class="text-white hover:bg-white/20 rounded-full p-2 transition-colors"
                            >
                                "✕"
                            </button>
                        </div>

                        // Modal Body
                        <div class="p-8">
                            <form class="space-y-6">
                                
                                // Event Title
                                <div>
                                    <label class="block text-sm font-semibold text-gray-700 mb-2">
                                        "Event Title"
                                    </label>
                                    <input
                                        type="text"
                                        placeholder="e.g., Sunday Worship Service"
                                        class="w-full px-4 py-3 border-2 border-gray-200 rounded-xl focus:border-amber-500 focus:ring-4 focus:ring-amber-100 transition-all outline-none"
                                    />
                                </div>

                                // Event Date
                                <div class="grid grid-cols-2 gap-4">
                                    <div>
                                        <label class="block text-sm font-semibold text-gray-700 mb-2">
                                            "Date"
                                        </label>
                                        <input
                                            type="date"
                                            class="w-full px-4 py-3 border-2 border-gray-200 rounded-xl focus:border-amber-500 focus:ring-4 focus:ring-amber-100 transition-all outline-none"
                                        />
                                    </div>
                                    <div>
                                        <label class="block text-sm font-semibold text-gray-700 mb-2">
                                            "Time"
                                        </label>
                                        <input
                                            type="time"
                                            class="w-full px-4 py-3 border-2 border-gray-200 rounded-xl focus:border-amber-500 focus:ring-4 focus:ring-amber-100 transition-all outline-none"
                                        />
                                    </div>
                                </div>

                                // Event Location
                                <div>
                                    <label class="block text-sm font-semibold text-gray-700 mb-2">
                                        "Location"
                                    </label>
                                    <input
                                        type="text"
                                        placeholder="e.g., Main Sanctuary"
                                        class="w-full px-4 py-3 border-2 border-gray-200 rounded-xl focus:border-amber-500 focus:ring-4 focus:ring-amber-100 transition-all outline-none"
                                    />
                                </div>

                                // Event Description
                                <div>
                                    <label class="block text-sm font-semibold text-gray-700 mb-2">
                                        "Description"
                                    </label>
                                    <textarea
                                        rows="4"
                                        placeholder="Describe the event..."
                                        class="w-full px-4 py-3 border-2 border-gray-200 rounded-xl focus:border-amber-500 focus:ring-4 focus:ring-amber-100 transition-all outline-none resize-none"
                                    ></textarea>
                                </div>

                                // Event Category
                                <div>
                                    <label class="block text-sm font-semibold text-gray-700 mb-2">
                                        "Category"
                                    </label>
                                    <select class="w-full px-4 py-3 border-2 border-gray-200 rounded-xl focus:border-amber-500 focus:ring-4 focus:ring-amber-100 transition-all outline-none">
                                        <option>"Worship Service"</option>
                                        <option>"Bible Study"</option>
                                        <option>"Prayer Meeting"</option>
                                        <option>"Youth Event"</option>
                                        <option>"Community Outreach"</option>
                                        <option>"Special Event"</option>
                                    </select>
                                </div>

                                // Action Buttons
                                <div class="flex gap-4 pt-4">
                                    <button
                                        type="button"
                                        on:click=move |_| set_show_event_modal.set(false)
                                        class="flex-1 py-3 border-2 border-gray-300 text-gray-700 font-semibold rounded-xl hover:bg-gray-50 transition-all"
                                    >
                                        "Cancel"
                                    </button>
                                    <button
                                        type="submit"
                                        class="flex-1 py-3 bg-gradient-to-r from-amber-600 to-amber-700 text-white font-semibold rounded-xl hover:shadow-lg hover:scale-[1.02] transition-all"
                                    >
                                        "Create Event"
                                    </button>
                                </div>

                            </form>
                        </div>

                    </div>
                </div>
            })}

            // Quote Modal
            {move || show_quote_modal.get().then(|| view! {
                <div class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-6">
                    <div class="bg-white rounded-3xl shadow-2xl max-w-2xl w-full max-h-[90vh] overflow-y-auto">
                        
                        // Modal Header
                        <div class="bg-gradient-to-r from-rose-600 to-rose-700 px-8 py-6 flex items-center justify-between">
                            <h2 class="text-2xl font-bold text-white">"Add Daily Quote"</h2>
                            <button 
                                on:click=move |_| set_show_quote_modal.set(false)
                                class="text-white hover:bg-white/20 rounded-full p-2 transition-colors"
                            >
                                "✕"
                            </button>
                        </div>

                        // Modal Body
                        <div class="p-8">
                            <form class="space-y-6">
                                
                                // Quote Text
                                <div>
                                    <label class="block text-sm font-semibold text-gray-700 mb-2">
                                        "Quote Text"
                                    </label>
                                    <textarea
                                        rows="4"
                                        placeholder="Enter the inspirational quote or Bible verse..."
                                        class="w-full px-4 py-3 border-2 border-gray-200 rounded-xl focus:border-rose-500 focus:ring-4 focus:ring-rose-100 transition-all outline-none resize-none"
                                    ></textarea>
                                </div>

                                // Bible Reference
                                <div>
                                    <label class="block text-sm font-semibold text-gray-700 mb-2">
                                        "Bible Reference"
                                    </label>
                                    <input
                                        type="text"
                                        placeholder="e.g., Psalm 23:1"
                                        class="w-full px-4 py-3 border-2 border-gray-200 rounded-xl focus:border-rose-500 focus:ring-4 focus:ring-rose-100 transition-all outline-none"
                                    />
                                </div>

                                // Author (Optional)
                                <div>
                                    <label class="block text-sm font-semibold text-gray-700 mb-2">
                                        "Author (Optional)"
                                    </label>
                                    <input
                                        type="text"
                                        placeholder="e.g., Pastor John"
                                        class="w-full px-4 py-3 border-2 border-gray-200 rounded-xl focus:border-rose-500 focus:ring-4 focus:ring-rose-100 transition-all outline-none"
                                    />
                                </div>

                                // Publish Date
                                <div>
                                    <label class="block text-sm font-semibold text-gray-700 mb-2">
                                        "Publish Date"
                                    </label>
                                    <input
                                        type="date"
                                        class="w-full px-4 py-3 border-2 border-gray-200 rounded-xl focus:border-rose-500 focus:ring-4 focus:ring-rose-100 transition-all outline-none"
                                    />
                                </div>

                                // Category
                                <div>
                                    <label class="block text-sm font-semibold text-gray-700 mb-2">
                                        "Category"
                                    </label>
                                    <select class="w-full px-4 py-3 border-2 border-gray-200 rounded-xl focus:border-rose-500 focus:ring-4 focus:ring-rose-100 transition-all outline-none">
                                        <option>"Faith"</option>
                                        <option>"Hope"</option>
                                        <option>"Love"</option>
                                        <option>"Prayer"</option>
                                        <option>"Wisdom"</option>
                                        <option>"Encouragement"</option>
                                    </select>
                                </div>

                                // Action Buttons
                                <div class="flex gap-4 pt-4">
                                    <button
                                        type="button"
                                        on:click=move |_| set_show_quote_modal.set(false)
                                        class="flex-1 py-3 border-2 border-gray-300 text-gray-700 font-semibold rounded-xl hover:bg-gray-50 transition-all"
                                    >
                                        "Cancel"
                                    </button>
                                    <button
                                        type="submit"
                                        class="flex-1 py-3 bg-gradient-to-r from-rose-600 to-rose-700 text-white font-semibold rounded-xl hover:shadow-lg hover:scale-[1.02] transition-all"
                                    >
                                        "Publish Quote"
                                    </button>
                                </div>

                            </form>
                        </div>

                    </div>
                </div>
            })}

        </div>
    }
}

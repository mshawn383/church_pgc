use leptos::prelude::*;

#[component]
pub fn Login() -> impl IntoView {
    view! {
        <div class="relative w-full min-h-screen bg-gradient-to-br from-amber-50 via-white to-rose-50 flex items-center justify-center overflow-hidden">
            
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

            // Login Card
            <div class="relative z-10 w-full max-w-md px-6">
                <div class="bg-white/90 backdrop-blur-lg rounded-3xl shadow-2xl border border-white/50 overflow-hidden">
                    
                    // Header Section
                    <div class="bg-gradient-to-r from-amber-600 to-rose-600 px-8 py-10 text-center">
                        <div class="text-5xl mb-4">"🕊️"</div>
                        <h1 class="text-3xl font-bold text-white mb-2">"Welcome Back"</h1>
                        <p class="text-amber-50 text-sm">"Sign in to continue your spiritual journey"</p>
                    </div>

                    // Form Section
                    <div class="px-8 py-10">
                        
                        <form class="space-y-6">
                            
                            // Email Input
                            <div>
                                <label for="email" class="block text-sm font-semibold text-gray-700 mb-2">
                                    "Email Address"
                                </label>
                                <div class="relative">
                                    <div class="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
                                        <span class="text-gray-400">"✉️"</span>
                                    </div>
                                    <input
                                        type="email"
                                        id="email"
                                        placeholder="your.email@example.com"
                                        class="w-full pl-12 pr-4 py-3 border-2 border-gray-200 rounded-xl focus:border-amber-500 focus:ring-4 focus:ring-amber-100 transition-all outline-none text-gray-700"
                                    />
                                </div>
                            </div>

                            // Password Input
                            <div>
                                <label for="password" class="block text-sm font-semibold text-gray-700 mb-2">
                                    "Password"
                                </label>
                                <div class="relative">
                                    <div class="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
                                        <span class="text-gray-400">"🔒"</span>
                                    </div>
                                    <input
                                        type="password"
                                        id="password"
                                        placeholder="Enter your password"
                                        class="w-full pl-12 pr-4 py-3 border-2 border-gray-200 rounded-xl focus:border-amber-500 focus:ring-4 focus:ring-amber-100 transition-all outline-none text-gray-700"
                                    />
                                </div>
                            </div>

                            // Remember Me & Forgot Password
                            <div class="flex items-center justify-between text-sm">
                                <label class="flex items-center gap-2 cursor-pointer">
                                    <input type="checkbox" class="w-4 h-4 text-amber-600 border-gray-300 rounded focus:ring-amber-500" />
                                    <span class="text-gray-600">"Remember me"</span>
                                </label>
                                <a href="#" class="text-amber-600 hover:text-amber-700 font-medium transition">
                                    "Forgot password?"
                                </a>
                            </div>

                            // Login Button
                            <button
                                type="submit"
                                class="w-full py-4 bg-gradient-to-r from-amber-600 to-rose-600 text-white font-bold rounded-xl hover:shadow-xl hover:scale-[1.02] transition-all duration-300 flex items-center justify-center gap-2"
                            >
                                <span>"Sign In"</span>
                                <span>"→"</span>
                            </button>

                        </form>

                        // Divider
                        <div class="relative my-8">
                            <div class="absolute inset-0 flex items-center">
                                <div class="w-full border-t border-gray-200"></div>
                            </div>
                            <div class="relative flex justify-center text-sm">
                                <span class="px-4 bg-white text-gray-500">"Or continue with"</span>
                            </div>
                        </div>

                        // Social Login Buttons
                        <div class="grid grid-cols-2 gap-4">
                            <button class="flex items-center justify-center gap-2 px-4 py-3 border-2 border-gray-200 rounded-xl hover:bg-gray-50 hover:border-gray-300 transition-all">
                                <span>"🔵"</span>
                                <span class="text-gray-700 font-medium text-sm">"Google"</span>
                            </button>
                            <button class="flex items-center justify-center gap-2 px-4 py-3 border-2 border-gray-200 rounded-xl hover:bg-gray-50 hover:border-gray-300 transition-all">
                                <span>"📘"</span>
                                <span class="text-gray-700 font-medium text-sm">"Facebook"</span>
                            </button>
                        </div>

                        // Sign Up Link
                        <div class="mt-8 text-center">
                            <p class="text-gray-600 text-sm">
                                "Don't have an account? "
                                <a href="#" class="text-amber-600 hover:text-amber-700 font-semibold transition">
                                    "Sign up here"
                                </a>
                            </p>
                        </div>

                    </div>

                    // Footer Quote
                    <div class="bg-gradient-to-r from-amber-50 to-rose-50 px-8 py-6 border-t border-gray-100">
                        <p class="text-center text-sm text-gray-600 italic">
                            "\"I am the way, the truth, and the life.\" - John 14:6"
                        </p>
                    </div>

                </div>

                // Back to Home Link
                <div class="mt-6 text-center">
                    <a href="/" class="inline-flex items-center gap-2 text-gray-600 hover:text-amber-600 transition font-medium">
                        <span>"←"</span>
                        <span>"Back to Home"</span>
                    </a>
                </div>

            </div>

        </div>
    }
}

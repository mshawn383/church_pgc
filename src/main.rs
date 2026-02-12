use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
mod components;
mod supabase;

use components::daily_quotes::DailyQuotes;
use components::dashboard::Dashboard;
use components::events::Events;
use components::gallery::Gallery;
use components::home::Home;
use components::login::Login;
use components::navigation::Navigation;
use supabase::auth::Authentication;

fn main() {
    mount_to_body(|| {
        view! {
        <Router>
         <Navigation />
                  <Routes fallback=|| "Not found.">
                      <Route path=path!("/") view=Home />
                      <Route path=path!("/login") view=Login />
                      <Route path=path!("/dashboard") view=Dashboard />
                      <Route path=path!("/daily-quotes") view=DailyQuotes />
                      <Route path=path!("/events") view=Events />
                      <Route path=path!("/gallery") view=Gallery />
                  </Routes>
              </Router>

                 }
    });
}

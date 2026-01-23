use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
mod components;

use components::daily_quotes::DailyQuotes;
use components::events::Events;
use components::home::Home;
use components::navigation::Navigation;

use components::gallery::Gallery;

fn main() {
    mount_to_body(|| {
        view! {
        <Router>
         <Navigation />
                  <Routes fallback=|| "Not found.">
                      <Route path=path!("/") view=Home />
                      <Route path=path!("/daily-quotes") view=DailyQuotes />
                      <Route path=path!("/events") view=Events />
                      <Route path=path!("/gallery") view=Gallery />
                  </Routes>
              </Router>

                 }
    });
}

use leptos::prelude::*;
use leptos::task::spawn_local;
use super::{supabase_login, SupabaseSession};

#[component]
pub fn Authentication() -> impl IntoView {
    let (email,set_email) = signal(String::new());
    let (password,set_password) = signal(String::new());
    let (session,set_session) = signal(None::<SupabaseSession>);

    let handle_login = move |_| {
        let email = email.get();
        let password = password.get();

        spawn_local(async move {
            let result = supabase_login(
                "http://192.168.56.101:8000",
                "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyAgCiAgICAicm9sZSI6ICJhbm9uIiwKICAgICJpc3MiOiAic3VwYWJhc2UtZGVtbyIsCiAgICAiaWF0IjogMTY0MTc2OTIwMCwKICAgICJleHAiOiAxNzk5NTM1NjAwCn0.dc_X5iR_VP_qT0zsiyj_I_OZ2T9FtRU2BBNWN8Bu4GE",
                &email,
                &password,
            )
            .await;

            if let Ok(sess) = result {
                set_session.set(Some(sess));
            }
        });
    };

    view! {
        <div>
            <input type="email" on:input=move |ev| set_email.set(event_target_value(&ev)) />
            <input type="password" on:input=move |ev| set_password.set(event_target_value(&ev)) />
            <button on:click=handle_login>"Login"</button>

            {move || session.get().map(|s| view! {
                <p>"Logged in. Token: " {s.access_token.clone()}</p>
            })}
        </div>
    }
}

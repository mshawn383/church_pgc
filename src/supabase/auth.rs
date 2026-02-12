use leptos::prelude::*;
#[component]
pub fn Login() -> impl IntoView {
    let email = signal(String::new());
    let password = signal(String::new());
    let session = signal(None::<SupabaseSession>);

    let login_action = Action::new(move |_| {
        let email = email.get();
        let password = password.get();

        async move {
            let result = supabase_login(
                "https://YOUR_PROJECT.supabase.co",
                "YOUR_PUBLIC_ANON_KEY",
                &email,
                &password,
            )
            .await;

            if let Ok(sess) = result {
                session.set(Some(sess));
            }
        }
    });

    view! {
        <div>
            <input type="email" on:input=move |ev| email.set(event_target_value(&ev)) />
            <input type="password" on:input=move |ev| password.set(event_target_value(&ev)) />
            <button on:click=move |_| login_action.dispatch(())>"Login"</button>

            {move || session.get().map(|s| view! {
                <p>"Logged in. Token: " {s.access_token.clone()}</p>
            })}
        </div>
    }
}
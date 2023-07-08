use ethers::prelude::*;
use gloo_console::log;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    let counter = use_state(|| 0);

    let address = use_state(|| Address::zero());
    let balance = use_state(|| U256::zero());

    let onclick = {
        let counter = counter.clone();

        log!("counter", JsValue::from(*counter));
        move |_| {
            spawn_local(async move {
                let provider = Provider::new(Eip1193::new());

                let accounts = provider.request_accounts().await.unwrap();
                // iterate over the accounts and print them
                for acc in accounts.iter() {
                    // if the first account, then set the address
                    // if acc == &accounts[0] {
                    //     address.set(*acc);
                    // }
                    log!("account", JsValue::from(acc.to_string()));
                }

                // Set the balance of the first account
                let balance = provider.get_balance(accounts[0], None).await.unwrap();
                log!("balance", JsValue::from(balance.to_string()));
            });

            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "Metamask Me 🖱️" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

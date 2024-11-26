use yew::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use reqwest::Client;
use web_sys::HtmlInputElement;
use wasm_bindgen_futures::spawn_local;

#[derive(Serialize, Deserialize)]
struct Expense {
    id: Uuid,
    amount: f64,
    description: String,
    date: String,
    mileage: i32,
    price: f64,
}

#[function_component(ExpenseForm)]
pub fn expense_form() -> Html {
    let amount = use_state(|| 0.0);
    let description = use_state(|| "".to_string());
    let date = use_state(|| "".to_string());
    let mileage = use_state(|| 0);
    let price = use_state(|| 0.0);
    let message = use_state(|| "".to_string());

    let onsubmit = {
        let amount = amount.clone();
        let description = description.clone();
        let date = date.clone();
        let mileage = mileage.clone();
        let price = price.clone();
        let message = message.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let expense = Expense {
                id: Uuid::new_v4(),
                amount: *amount,
                description: (*description).clone(),
                date: (*date).clone(),
                mileage: *mileage,
                price: *price,
            };

            let message = message.clone();
            spawn_local(async move {
                let client = Client::new();
                match client.post("http://127.0.0.1:8081/expenses/")
                    .json(&expense)
                    .send()
                    .await {
                        Ok(response) => {
                            if response.status().is_success() {
                                message.set("Expense added successfully".to_string());
                            } else {
                                let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                                message.set(format!("Failed to add expense: {}", error_text));
                            }
                        },
                        Err(err) => {
                            message.set(format!("Error: Failed to add expense: {}", err));
                        }
                    }
            });
        })
    };

    html! {
        <form {onsubmit}>
            <div>
                <label for="amount">{ "Amount: " }</label>
                <input type="number" id="amount" value={amount.to_string()} oninput={Callback::from(move |e: InputEvent| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    amount.set(input.value_as_number());
                })} />
            </div>
            <div>
                <label for="description">{ "Description: " }</label>
                <input type="text" id="description" value={description.to_string()} oninput={Callback::from(move |e: InputEvent| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    description.set(input.value());
                })} />
            </div>
            <div>
                <label for="date">{ "Date: " }</label>
                <input type="date" id="date" value={date.to_string()} oninput={Callback::from(move |e: InputEvent| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    date.set(input.value());
                })} />
            </div>
            <div>
                <label for="mileage">{ "Mileage: " }</label>
                <input type="number" id="mileage" value={mileage.to_string()} oninput={Callback::from(move |e: InputEvent| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    mileage.set(input.value_as_number() as i32);
                })} />
            </div>
            <div>
                <label for="price">{ "Price: " }</label>
                <input type="number" id="price" value={price.to_string()} oninput={Callback::from(move |e: InputEvent| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    price.set(input.value_as_number());
                })} />
            </div>
            <button type="submit">{ "Create Expense" }</button>
            <p>{ (*message).clone() }</p>
        </form>
    }
}
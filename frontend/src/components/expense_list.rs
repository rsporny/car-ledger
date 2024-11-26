use yew::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use reqwest::Client;
use wasm_bindgen_futures::spawn_local;
use log::{info, error};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Expense {
    id: Uuid,
    amount: f64,
    description: String,
    date: String,
    mileage: i32,
    price: f64,
}

#[function_component(ExpenseList)]
pub fn expense_list() -> Html {
    let expenses = use_state(|| vec![]);

    {
        let expenses = expenses.clone();
        use_effect_with_deps(move |_| {
            spawn_local(async move {
                let client = Client::new();
                match client.get("http://127.0.0.1:8081/expenses/").send().await {
                    Ok(response) => {
                        if response.status().is_success() {
                            match response.json::<Vec<Expense>>().await {
                                Ok(data) => {
                                    info!("Fetched expenses: {:?}", data);
                                    expenses.set(data);
                                }
                                Err(err) => {
                                    error!("Failed to parse response: {:?}", err);
                                }
                            }
                        } else {
                            error!("Failed to fetch expenses: {:?}", response.status());
                        }
                    }
                    Err(err) => {
                        error!("Request error: {:?}", err);
                    }
                }
            });
            || ()
        }, ());
    }

    html! {
        <div>
            <h1>{ "Expenses" }</h1>
            <ul>
                { for expenses.iter().map(|expense| html! {
                    <li key={expense.id.to_string()}>
                        { format!("{} - {} - {} - {} - {} - {}", expense.id, expense.amount, expense.description, expense.date, expense.mileage, expense.price) }
                    </li>
                }) }
            </ul>
        </div>
    }
}
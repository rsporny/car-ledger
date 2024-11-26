use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{ExpenseForm, ExpenseList};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/expenses")]
    Expenses,
    #[at("/expenses/new")]
    NewExpense,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Welcome to Car Ledger" }</h1> },
        Route::Expenses => html! { <ExpenseList /> },
        Route::NewExpense => html! { <ExpenseForm /> },
    }
}
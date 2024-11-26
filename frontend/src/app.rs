use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::{switch, Route};

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <div class="container">
                    <nav>
                        <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                        <Link<Route> to={Route::Expenses}>{ "Expenses" }</Link<Route>>
                        <Link<Route> to={Route::NewExpense}>{ "New Expense" }</Link<Route>>
                    </nav>
                    <Switch<Route> render={switch} />
                </div>
            </BrowserRouter>
        }
    }
}
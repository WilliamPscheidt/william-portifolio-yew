use yew::prelude::*;

#[function_component]
pub fn HeaderComponent() -> Html {
    html! {
        <header>
            <ul>
                <li>{"/William"}</li>
                <span>{"·"}</span>
                <li>{"/Projetos"}</li>
                <span>{"·"}</span>
                <li>{"/Contato"}</li>
            </ul>
            <h1>{"William Pscheidt Polaski"}</h1>
        </header>
    }
}
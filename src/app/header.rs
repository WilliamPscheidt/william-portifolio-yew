use yew::prelude::*;

#[function_component]
pub fn HeaderComponent() -> Html {
    html! {
        <header>
            <nav class={"header"}>
                <a href={"#"}>{"/William"}</a>
                <span id={"span-header"}>{"·"}</span>
                <a href={"#"}>{"/Projetos"}</a>
                <span id={"span-header"}>{"·"}</span>
                <a href={"#"}>{"/Contato"}</a>
            </nav>
            <h1 class={"my-name"}>{"William Pscheidt Polaski"}</h1>
        </header>
    }
}
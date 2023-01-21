use yew::prelude::*;

#[function_component]
pub fn FooterComponent() -> Html {
    html! {
        <footer class={"footer"}>
            <h4 class={"footer-text"}>{"Criado com"}</h4>
            <span style={"color: red; padding: 0 5px;"}>{" ❤ "}</span>
            <h4 class={"footer-text"}>{"por William Pscheidt"}</h4>
        </footer>
    }
}
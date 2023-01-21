use yew::prelude::*;

#[function_component]
pub fn Contato() -> Html {
    html! {
        <section class={"contato"}>
            <h3 class={"title"}>
                {"Contato"}
            </h3>
            <div class={"social-media"}>
                <img src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/linkedin/linkedin-original.svg" />
                <span id={"separator"}>{"·"}</span>
                <a href="https://www.linkedin.com/in/william-pscheidt-9193341a4/">{"Acessar"}</a>
            </div>
            
        </section>
    }
}
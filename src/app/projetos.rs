use yew::prelude::*;

#[function_component]
pub fn Projetos() -> Html {
    html! {
        <section class={"projetos"}>
            <span>{"Projetos"}</span>
            <p>{"Minha lista de projetos mais recentes:"}</p>
        </section>
    }
}
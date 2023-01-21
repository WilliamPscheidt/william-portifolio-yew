use yew::prelude::*;

#[function_component]
pub fn Projetos() -> Html {
    html! {
        <section class={"projetos"}>
            <h3 class={"title"}>
                {"Projetos"}
            </h3>
            <p>{"Minha lista de projetos mais recentes:"}</p>
        </section>
    }
}
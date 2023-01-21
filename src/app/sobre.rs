use yew::prelude::*;

#[function_component]
pub fn Sobre() -> Html {
    html! {
        <section class={"sobre"}>
            <span>{"Sobre"}</span>
            <p>{"Olá, meu nome é William, trabalho como analista de suporte e atualmente o meu foco está sendo totalmente no desenvolvimento Back-end. "}</p>
        </section>
    }
}
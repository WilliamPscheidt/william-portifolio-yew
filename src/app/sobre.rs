use yew::prelude::*;

#[function_component]
pub fn Sobre() -> Html {
    html! {
        <section class={"sobre"}>
            <h3 class={"title"}>
                {"Sobre"}
            </h3>
            <p>{"Olá, meu nome é William e atuo como analista de suporte. No entanto, meu foco atual tem sido direcionado para o desenvolvimento de soluções Back-end. Acredito que o desenvolvimento Back-end é fundamental para o sucesso de qualquer projeto, pois é responsável por garantir a estabilidade e a escalabilidade da aplicação. Além disso, trabalhar com Back-end me dá a oportunidade de trabalhar com tecnologias avançadas e com uma equipe de desenvolvedores altamente capacitados. Estou constantemente buscando aprender mais sobre as melhores práticas de desenvolvimento e tendências do mercado para que eu possa contribuir para a criação de soluções cada vez mais eficientes e inovadoras. "}</p>
        </section>
    }
}
use yew::prelude::*;
mod header;
mod sobre;
mod skills;
mod projetos;
mod contato;
mod footer;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <div class={"content"}>
                <header::HeaderComponent/>
                <sobre::Sobre/>
                <skills::Skills/>
                <projetos::Projetos/>
                <contato::Contato/>
            </div>
            <footer::FooterComponent/>
        </main>
    }
}

use yew::prelude::*;

#[function_component]
pub fn Projetos() -> Html {
    html! {
        <section class={"projetos"}>
            <h3 class={"title"}>
                {"Projetos"}
            </h3>
            <ul>
                <li>
                    <a href="https://github.com/WilliamPscheidt/lsw-windows">{"lsw project"}</a>
                    {" Rust"}
                </li>
                <li>
                    <a href="https://github.com/WilliamPscheidt/william-portifolio">{"william-portifolio"}</a>
                    {" Yew & Rust stack"}
                </li>
                <li>
                    <a href="https://github.com/WilliamPscheidt/Ecommerce-API">{"Ecommerce-API"}</a>
                    {" Node.JS & MongoDB"}
                </li>
                <li>
                    <a href="https://github.com/WilliamPscheidt/Ecommerce-frontend">{"Ecommerce-frontend"}</a>
                    {" React.JS"}
                </li>
                <li>
                    <a href="https://github.com/WilliamPscheidt/Space-Agencys">{"Space-Agencys"}</a>
                    {" React"}
                </li>
                <li>
                    <a href="https://github.com/WilliamPscheidt/Questions-Platform">{"Questions-Platform"}</a>
                    {" Node.JS & React"}
                </li>
                <li>
                    <a href="https://github.com/WilliamPscheidt/TS-Blog">{"TS-Blog"}</a>
                    {" TypeScript"}
                </li>
                <li>
                    <a href="https://github.com/WilliamPscheidt/Todo-list">{"Todo-list"}</a>
                    {" React"}
                </li>
                <li>
                    <a href="https://github.com/WilliamPscheidt/League-Of-Legends-API">{"League-Of-Legends-API"}</a>
                    {" Next.JS"}
                </li>
                <li>
                    <a href="https://github.com/WilliamPscheidt/Python-SnakeGame">{"Snakegame"}</a>
                    {" Python"}
                </li>
                <li>
                    <a href="https://github.com/WilliamPscheidt/Login-Register-API">{"Login-Register-API"}</a>
                    {" Node.JS"}
                </li>
                <li>
                    <a href="https://github.com/WilliamPscheidt/Web-hosting-template">{"Web-hosting-template"}</a>
                    {" React"}
                </li>
                <li>
                    <a href="https://github.com/WilliamPscheidt/Blog">{"Blog"}</a>
                    {" Node.JS"}
                </li>
                <li>
                    <a href="https://github.com/WilliamPscheidt/Plataforma-Simulados">{"Plataforma-Simulados"}</a>
                    {" Node.JS, MySQL & React"}
                </li>
                <li>
                    <a href="https://github.com/WilliamPscheidt/DNS-Search-Desktop">{"DNS-Search-Desktop"}</a>
                    {" Electron"}
                </li>
                <li>
                    <a href="https://github.com/WilliamPscheidt/DNS-Search">{"DNS-Search"}</a>
                    {" Html & JavaScript"}
                </li>
            </ul>
        </section>
    }
}
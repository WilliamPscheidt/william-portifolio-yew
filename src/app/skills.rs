use yew::prelude::*;

#[function_component]
pub fn Skills() -> Html {
    html! {
        <section class={"skills"}>
            <h3 class={"title"}>
                {"Principais Skills"}
            </h3>
            <p>{"Atualmente minhas principais habilidades são em:"}</p>

            <div class={"skills_"}>
                <div class="technology">
                    <img id={"skill"} src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/javascript/javascript-plain.svg"/>
                    <span id={"separator"}>{"·"}</span>
                    <span span={"tech"}>{"Javascript"}</span>
                </div>

                <div class="technology">
                    <img id={"skill"} src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/python/python-original.svg"/>
                    <span id={"separator"}>{"·"}</span>
                    <span span={"tech"}>{"Python"}</span>
                </div>

                <div class="technology">
                    <img id={"skill"} src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/rust/rust-plain.svg"/>
                    <span id={"separator"}>{"·"}</span>
                    <span span={"tech"}>{"Rust"}</span>
                </div>

                <div class="technology">
                    <img id={"skill"} src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/java/java-original.svg"/>
                    <span id={"separator"}>{"·"}</span>
                    <span span={"tech"}>{"Java"}</span>
                </div>

                <div class="technology">
                    <img id={"skill"} src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/html5/html5-original.svg"/>
                    <span id={"separator"}>{"·"}</span>
                    <span span={"tech"}>{"HTML-5"}</span>
                </div>

                <div class="technology">
                    <img id={"skill"} src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/css3/css3-original.svg"/>
                    <span id={"separator"}>{"·"}</span>
                    <span span={"tech"}>{"CSS3"}</span>
                </div>

                <div class="technology">
                    <img id={"skill"} src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/nodejs/nodejs-original.svg"/>
                    <span id={"separator"}>{"·"}</span>
                    <span span={"tech"}>{"Node.JS"}</span>
                </div>

                <div class="technology">
                    <img id={"skill"} src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/react/react-original.svg"/>
                    <span id={"separator"}>{"·"}</span>
                    <span span={"tech"}>{"Ract.JS"}</span>
                </div>

                <div class="technology">
                    <img id={"skill"} src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/nextjs/nextjs-original.svg"/>
                    <span id={"separator"}>{"·"}</span>
                    <span span={"tech"}>{"Next.JS"}</span>
                </div>

                <div class="technology">
                    <img id={"skill"} src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/nginx/nginx-original.svg"/>
                    <span id={"separator"}>{"·"}</span>
                    <span span={"tech"}>{"Nginx"}</span>
                </div>

                <div class="technology">
                    <img id={"skill"} src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/linux/linux-original.svg"/>
                    <span id={"separator"}>{"·"}</span>
                    <span span={"tech"}>{"Linux"}</span>
                </div>
                    
                <div class="technology">
                    <img id={"skill"} src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/git/git-original.svg"/>
                    <span id={"separator"}>{"·"}</span>
                    <span span={"tech"}>{"GIT"}</span>
                </div>

                <div class="technology">
                    <img id={"skill"} src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/mongodb/mongodb-original.svg"/>
                    <span id={"separator"}>{"·"}</span>
                    <span span={"tech"}>{"MongoDB"}</span>
                </div>
                
                <div class="technology">
                    <img id={"skill"} src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/mysql/mysql-original.svg"/>
                    <span id={"separator"}>{"·"}</span>
                    <span span={"tech"}>{"MySQL"}</span>
                </div>

            </div>

        </section>
    }
}

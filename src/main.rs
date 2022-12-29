use yew::{function_component, html};

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
fn app_body() -> yew::Html {
    html! {
        html! {
        <main>
            <head>
                <link href="./css/estilos.css" rel="stylesheet"/>
            </head>
                <div class="centro">
                    <h1 class="ejemplo">{ "Bienvenido a Salamandra Devs!" }</h1>
                </div>
            <img src="./img/salamandra.jpeg" class ="salamandra" alt="Logo GM" style="width: 150px; height: 74px;"/>
            <p class="texto">{ "Salamandra Devs es una comunidad de desarrolladores de software, donde puedes aprender y compartir tus conocimientos." }</p>
        </main>
        }
    }
}

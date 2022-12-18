use yew::{function_component, html};

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
fn app() -> Html {
    html! {
        html! {
        <main>
            <head>
                <link href="./css/estilos.css" rel="stylesheet"/>
            </head>
                <div class="centro">
                    <h1 class="ejemplo">{ "Bienvenido a Salamandra Devs!" }</h1>
                </div>
        </main>
        }
    }
}

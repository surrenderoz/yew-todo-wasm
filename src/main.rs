
use yew::{prelude::*};
use stylist::yew::use_style;
use yew_router::prelude::*;


mod component;

use component::add_todo::AddTodo;
use component::view_todo::ViewTodo;
use web_sys::console::{self, log_1};


use wasm_bindgen::{JsCast};
use web_sys::{ HtmlInputElement, HtmlElement};


#[derive(PartialEq, Properties, Clone)]
struct Props {
    // #[prop_or_default]
    messages: String,
    onload: Callback<String>
}

#[function_component]
fn Hello(props: &Props)  -> Html{
    // let Props { messages, onload  } = props;

    let message: UseStateHandle<String> = use_state(|| "default".to_owned());
    let cln = message.clone();
    let propmsg = String::from(&props.messages);
    let onclick = Callback::from(move |event: MouseEvent| {
        console::log_1(&"helo".into());
       cln.set(String::from(&propmsg));
    });


    let style = use_style!(
        "
        .ok {
            color: red;
        };
        
        color: red;
        background-color: black;
        padding: 10px;
        cursor: pointer;
        :hover {
        background-color: green;
        }
         ",
    );
    &props.onload.emit("Calling back".to_owned());
    let cloneedsmg = message.clone();


    let onchange = Callback::from(move |event: Event| {
        let msg = event.target().unwrap().unchecked_into::<HtmlInputElement>();
        console::log_1(&msg.value().into());
        cloneedsmg.set(String::from(&msg.value()));
    });
    
    let handlesubmit = Callback::from(|event: SubmitEvent| {
        let _ = &event.prevent_default();
        let v = &event.target().unwrap().unchecked_into::<HtmlElement>().children();
        log_1(&v.get_with_index(0).unwrap().unchecked_into::<HtmlInputElement>().value().into());
    });


    // console::log_1(&hmm.un);

    html! {
        <div class={style}>
        <h1 class="ok">{&*message}</h1>
        <input placeholder="text" onchange={onchange} /> 
        <button onclick={onclick}>{"SetN"}</button>
        <form onsubmit={handlesubmit}>
        
            <input placeholder="name"/>
            <input type="submit" value="sub" />
        </form>
        </div>
    }
}


#[derive(Clone, Routable, PartialEq)]
enum Route {
    
    #[at("/addtodo")]
    AddTodo,
    #[at("/")]
    ViewTodo,
    #[not_found]
    #[at("/404")]
    NotFound,
}


#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::AddTodo => html! { <AddTodo />},
        Route::ViewTodo => html! { <ViewTodo />},
        Route::NotFound => html! { <h1>{ "404" }</h1> }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
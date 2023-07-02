use stylist::yew::use_style;
use web_sys::{window, console};
use yew::prelude::*;
use yew_router::{prelude::use_navigator, Routable};
use crate::component::utils::fixedcomp::Wrapper;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/addtodo")]
    AddTodo
} 

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub  name: String,
    pub  id: u128
}
#[function_component]
pub fn ViewTodo() -> Html {

    // let todolist = use_state(|| vec![Todo{name: "om".to_owned(), id: 1}]);
    let storage = &window().unwrap().local_storage().unwrap().unwrap();
    let todo = &storage.get_item("Todo").unwrap().unwrap();
    let stringify: Vec<Todo> = serde_json::from_str(todo).unwrap();

    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::AddTodo));

    let styleit = use_style!("

        height: 100vh;
        display: flex;
        flex-direction: column;
        align-items: center;
        row-gap: 30px;
    
        
        h1 {
            text-align: center;
            color: white;
            margin: 30px 0;
        }
        .list {
            width: 100%;
        }
        .items {
            box-sizing: border-box;

            background-color: #fec7c7;
            padding: 10px;
            border-radius: 10px;
            color: white;
            font-weight: 600;  
            cursor: pointer;
            margin: 10px 0;
            width: 100%;
            box-shadow: -3px -4px 5px 1px #a5a1a1;
        }
        .items:hover {
            background: #fec7c;
        }
        .button { 
            position: relative;
            padding: 20px;
            border-radius: 100%;
            width: 100px;
            height: 100px;
            border: none;
            cursor: pointer;
            font-weight: 600;
            color: white;
            background: #ffc7c8;
            box-shadow: 0px 0px 9px 1px #a68f8f;
            
        }
        .button::after {
            content: '';
            border: 1px solid #f8f4f4;
            height: 70%;
            width: 70%;
            position: absolute;
            border-radius: 100%;
            top: 14px;
            left: 14px;
            margin: auto;
            padding: auto;
          
        }
        .button:hover::after {
            border-color: aqua;
        }
        .button:focus-within {
            background: red;
        }
    ");

    html!{
        <>
            <Wrapper>

                <div class={styleit}>
                <h1>{"Your Task List"}</h1>
                   if &stringify.len() > &1 {
                    <div class="list">
                        {
                            stringify.iter().map(|value| {
                                html!{
                                 <div class="items" key={value.id}>{&*value.name}</div>
                                }
                             }).collect::<Html>()
                        }
                    </div>
                    <button class="button" onclick={onclick}>{"New"}</button>
                   }
                   else {
                    <p>{"No Item Found"}</p>
                   }
                </div>
                
            </Wrapper>
        </>
        }
}
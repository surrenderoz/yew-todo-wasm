use js_sys::JSON;
use stylist::yew::use_style;
use web_sys::{console, window};
use yew::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json::{self, json};


use crate::component::utils::fixedcomp::{Wrapper, TextArea};

// use stylist::yew::use_style;
#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub name: String,
    pub id: u128
}

#[function_component]
pub fn AddTodo() -> Html {
    let item = use_state(|| "".to_owned());

    let items = item.clone();
   
    let handleonchange = Callback::from(move |event: String| {
        items.set(event.clone());
    });


    let handlesubmit = Callback::from(move |_| (
        {
            if &item.len() < &3 {
                return; 
            };
            let mut listdata: Vec<Todo> = vec![];
            let localstoreage = &window().unwrap().local_storage().unwrap().unwrap();
            let oldTodo = &localstoreage.get_item("Todo").unwrap().unwrap();
            
            let mut oldtdat: Vec<Todo> = serde_json::from_str(oldTodo).unwrap();

            let rm: Todo = {Todo{name: item.to_string(), id: 1}};
            listdata.push(rm);
            listdata.append(&mut oldtdat);

            let jsn = serde_json::to_string(&listdata).expect("msg");
            let _ = localstoreage.set_item("Todo", &jsn).clone();
        }
    ));

    let mainstyle = use_style!(
        "
        display: flex;
        justify-content: center;
        flex-direction: column;
        align-items: center;
        width: 100%;
        height: 100vh;
        row-gap: 30px;
        .text {
            margin: 10px 0;
            font-size: 26px;
            font-weight: 600;
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
        "
        );

    html!{
        <>
            <Wrapper>
                    <div class={mainstyle}>
                        <div class="text">{"Hey What's on your mind"}</div>
                        <TextArea onchange={handleonchange}/>
                        <button class="button" onclick={handlesubmit}>{"PUSH"}</button>
                        // <p>{&*item}</p>
                    </div>
            </Wrapper>
        </>
    }
}
use stylist::{yew::{use_style, styled_component}, };
use web_sys::{console, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
   pub children: Children,
}

#[styled_component]
pub fn Wrapper(props: &Props) -> Html {
    let wrapper = use_style!(
        "
            background-color: #03d5d5;
            padding: 20px;
            max-width: 600px;
            margin: auto;
            display: flex,
            justify-content: center;
            align-items: center;

        "
    );
    html!(
        <div class={wrapper}>
            {props.children.clone()}
        </div>
    )
}

#[derive(Properties, PartialEq, Clone)]
pub struct PropsTextArea {
    pub onchange: Callback<String>
}


#[function_component]
pub fn TextArea(props: &PropsTextArea) -> Html {

    let style = use_style!(
        "
        width: 100%;

        .textarea {
            width: 100%;
            height: 200px;
            border: none;
            text-decoration: none;
            border-radius: 5px;
            padding: 10px;
            box-sizing: border-box; 
        }  
        "
    );
        
    let sendcallback = props.onchange.clone();
    let onchange = {

        Callback::from(move |e: KeyboardEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let _ = &sendcallback.emit(input.value().clone());
        })
    };

    html!(
        <div class={style}>
            <textarea name={"addtext"} class="textarea" onkeyup={onchange} >
            </textarea>
        </div>
    )
}
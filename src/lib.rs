use gloo::console::log;
use serde::{Serialize, Deserialize};
use yew::prelude::*;

#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    favorite_language: String,
}


#[function_component]
pub fn App() -> Html {
    let name:&str = "Honorato";
    let my_object = MyObject{username: name.to_owned(), favorite_language:"Rust".to_owned()};


  //console functionality
   log!("My name is ",name);
   log!(serde_json::to_string_pretty(&my_object).unwrap());
   let mut class: &str = "My_title";
   let mut message: Option<&str> = None;

    html! {
        <>
          <div>
            
            <h1 class = {class}>{"Hello World"}</h1>
            if class == "My_titles" {
                <p>{"hi there"}</p>
            } else {
                <p>{"I'm not a titles"}</p>
            }

            if let Some(message) = message {
                <p>{message}</p>
            } else {
                <p>{"No message here bro!!"}</p>
            }
        </div>
        
        </>
      
    }
}
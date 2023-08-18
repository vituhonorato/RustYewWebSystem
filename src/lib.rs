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
    html! {
        <>
          <div>
            
            <h1>{"Hello World"}</h1>
        </div>
        
        </>
      
    }
}
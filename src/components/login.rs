use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <div class="bg-gradient-to-br from-blue-600 via-blue-400 to-blue-200 flex w-screen h-screen items-center justify-center">
          <div class="max-w-md w-full mx-auto p-8 bg-white rounded-lg shadow-lg transform transition duration-500 hover:scale-105">
            <form class="flex flex-col items-center">
              <h2 class="text-3xl font-bold mb-4 text-gray-800">{"Login to Chat"}</h2>
              <input {oninput} class="rounded-lg p-4 border border-gray-300 bg-gray-100 mb-4 w-full" placeholder="Username" />
              <Link<Route> to={Route::Chat}>
                <button {onclick} disabled={username.len() < 1} class="px-8 py-2 bg-blue-500 text-white font-bold rounded-lg shadow-md transition duration-300 transform hover:scale-105">
                  {"Go Chatting!"}
                </button>
              </Link<Route>>
            </form>
          </div>
        </div>
      }     
}

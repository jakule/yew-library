use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::prelude::*;
use web_sys;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/posts/:id")]
    Post { id: u64 },
    #[at("/posts")]
    Posts,
    #[at("/authors/:id")]
    Author { id: u64 },
    #[at("/authors")]
    Authors,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

enum Msg {
    AddOne,
    SubtractOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            Msg::SubtractOne => {
                self.value -= 1;

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{ "Hello World!" }</h1>
                <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>

                <button onclick={ctx.link().callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <button onclick={ctx.link().callback(|_| Msg::SubtractOne)}>{ "-1" }</button>
                <p>{ self.value }</p>
                <TestReq/>
            </div>
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Book {
    id: i32,
    title: String,
    authors: Vec<String>,
    publication_date: chrono::NaiveDate,
}

enum TestReqMsg {
    SetFetchState(Vec<Book>),
    Fetch,
}

struct TestReq {
    response: Vec<Book>,
}

impl From<Vec<Book>> for TestReqMsg {
    fn from(s: Vec<Book>) -> Self {
        TestReqMsg::SetFetchState(s)
    }
}

fn get_current_url() -> String {
    let window = web_sys::window().unwrap();
    let location = window.location();
    let host = location.host().unwrap();
    let protocol = location.protocol().unwrap();

    return format!("{}{}", protocol, host)
}

async fn fetch_data() -> Vec<Book> {
    let url = get_current_url();

    let resp = reqwest::get(&format!("{}/api/books", url)).await;
    return resp.unwrap().json().await.unwrap();
}

impl Component for TestReq {
    type Message = TestReqMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(TestReqMsg::Fetch);

        Self { response: vec![] }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TestReqMsg::Fetch => {
                ctx.link().send_future(async { fetch_data().await });

                false
            }
            TestReqMsg::SetFetchState(val) => {
                self.response = val;

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
          <div>
              <button type="button" class="btn btn-primary" onclick={ctx.link().callback(|_| TestReqMsg::Fetch)}>{"Get Books"}</button>
              <p>{"REST response:"}</p>
          <table class="table">
        <thead>
          <tr>
            <th scope="col">{"#"}</th>
            <th scope="col">{"Title"}</th>
            <th scope="col">{"Authors"}</th>
            <th scope="col">{"Publication Date"}</th>
          </tr>
        </thead>
        <tbody>
          {
              for self.response.iter().map(
          |e| html! {
              <tr>
                 <th scope="row">{e.id}</th>
                 <th>{&e.title}</th>
                 <th>{format!("{:?}", e.authors)}</th>
                 <th>{format!("{}", e.publication_date)}</th>
              </tr>
              }
            )
          }
          </tbody>
          </table>
          </div>
              }
    }
}

fn switch(routes: &Route) -> Html {
    // let onclick_callback = Callback::from(|_: Route| yew_router::push_route(Route::Home));
    match routes {
        Route::Home => html! { <Model/> },
        // Route::Secure => html! {
        //     <div>
        //         <h1>{ "Secure" }</h1>
        //         <button onclick={onclick_callback}>{ "Go Home" }</button>
        //     </div>
        // },
        Route::NotFound => html! { <h1>{ "404" }</h1> },

        _ => html! { <></> },
    }
}

struct Home {}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Home {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <Router<Route> render={Router::render(switch)} />
        }
    }
}

fn main() {
    web_sys::console::log_1(&"Start".into());
    yew::start_app::<Home>();
}

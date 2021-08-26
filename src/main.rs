use yew::prelude::*;

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

enum TestReqMsg {
    SetFetchState(String),
    Fetch,
}

struct TestReq {
    response: String,
}

impl From<String> for TestReqMsg {
    fn from(s: String) -> Self {
        TestReqMsg::SetFetchState(s)
    }
}

async fn fetch_data() -> String {
    let resp = reqwest::get("https://httpbin.org/ip").await;
    return resp.unwrap().text().await.unwrap();
}

impl Component for TestReq {
    type Message = TestReqMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            response: "".to_string(),
        }
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
                <button onclick={ctx.link().callback(|_| TestReqMsg::Fetch)}>{"Get IP"}</button>
                <p>{"REST response:"}</p>
                <p>{ self.response.clone() }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}

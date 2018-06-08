#![feature(proc_macro, generators)]

extern crate dotenv;
extern crate futures_await as futures;
extern crate telegram_bot;
extern crate tokio_core;

use dotenv::dotenv;
use futures::prelude::*;
use std::env;
use telegram_bot::prelude::*;
use telegram_bot::{Api, MessageKind, UpdateKind, UserId};
use tokio_core::reactor::Core;

const POSHEL_NAHUY: &str = "POSHEL NAHUY";
const ESSENCE_IN: &str = "не понимаешь сути";
const ESSENCE_OUT: &str = "
Ты СОВЕРШЕННО не понимаешь в чем суть расточата.
расточат это не стековерфлоу «у меня не запускается хелловорлд, напишите мне 2 листа причин».
расточат это не псевдоинтеллектуальные обсуждения на гитхабе.
расточат это не гиттер, IRC или сраный продот.
расточат это место, где люди могут побыть чудовищами — ужасными, бесчувственными,
безразличными чудовищами, которыми они на самом деле и являются.
В го нет дженериков, а мы смеемся.
Код питонистов падает в проде из-за косяков с типизацией, а мы смеемся.
Гоферы две недели не могут решить как писать простого бота в своем продоте, а мы смеемся и просим еще.
Утинная типизация, фризы GC, касты к void* — мы смеемся.
Тупые языки, национализм, дискриминация, ксенофобия, анальное рабство на интерпрайзных галерках,
беспричинная ненависть — мы смеемся.
Жаваскриптер  убил своего PMа чтобы не отлаживать легаси — мы смеемся.
Мы бездушно подпишемся под чем угодно, наши предпочтения не основаны на здравом смысле,
бесцельные споры — наша стихия, мы — истинное лицо IT-комьюнити.
";

fn main() {
    dotenv().ok();
    let mut core = Core::new().unwrap();
    let token = env::var("AUTORESPONSEBOT_TOKEN").unwrap();
    let api = Api::configure(token).build(core.handle()).unwrap();
    core.run(handle_updates(api)).unwrap();
}

#[async]
fn handle_updates(api: Api) -> Result<(), telegram_bot::Error> {
    let evengining_user = UserId::new(301800131); // @evengining
    let test_user = UserId::new(560120889); // test test
    let essence_disabled = vec![evengining_user, test_user];

    #[async]
    for update in api.stream() {
        if let UpdateKind::Message(message) = update.kind {
            let msg = message.clone();
            if let MessageKind::Text { data, .. } = message.kind {
                if data.to_lowercase().contains(ESSENCE_IN) {
                    if let Err(err) = await!(api.send(msg.text_reply(
                        if essence_disabled.contains(&message.from.id) {
                            POSHEL_NAHUY
                        } else {
                            ESSENCE_OUT
                        }
                    ))) {
                        println!("Failed to send message: {}", err);
                    }
                }
            }
        }
    }
    Ok(())
}

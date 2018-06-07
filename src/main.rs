extern crate dotenv;
extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;

use dotenv::dotenv;
use futures::Stream;
use std::env;
use telegram_bot::{Api, MessageKind, UpdateKind, UserId};
use telegram_bot::prelude::*;
use tokio_core::reactor::Core;

const POSHEL_NAHUY: &str = "POSHEL NAHUY";
const ESSENCE_IN: &str = "ты не понимаешь сути";
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

    let test_user = UserId::new(560120889);  // test test
    let mut core = Core::new().unwrap();
    let token = env::var("AUTORESPONSEBOT_TOKEN").unwrap();
    let api = Api::configure(token).build(core.handle()).unwrap();
    let future = api.stream().for_each(|update| {
        if let UpdateKind::Message(message) = update.kind {
            if message.from.id == test_user {
                api.spawn(message.text_reply(POSHEL_NAHUY));
            } else {
                if let MessageKind::Text { ref data, .. } = message.kind {
                    let data = data.to_lowercase();
                    if data.contains(ESSENCE_IN) {
                        api.spawn(message.text_reply(ESSENCE_OUT));
                    }
                }
            }
        }
        Ok(())
    });
    core.run(future).unwrap();
}

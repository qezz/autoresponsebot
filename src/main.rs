#![feature(proc_macro, generators)]
extern crate dotenv;
extern crate futures_await as futures;
extern crate regex;
extern crate telegram_bot;
extern crate tokio_core;

use dotenv::dotenv;
use futures::prelude::*;
use regex::Regex;
use std::env;
use telegram_bot::prelude::*;
use telegram_bot::{Api, Message, MessageEntityKind, MessageKind, SendMessage, UpdateKind, UserId};
use tokio_core::reactor::Core;

const BAD_WORDS_RE: &str = "[сsc][уuy][тt][ьиі1]?|[pпp][оo][нn][иiy1][мm][аa]|[pрr][оo][zsз][yuу][mм]|[uуаy][nн][дd][eе][рr]?[scс][tт][aэе][nн][dдт]?|[eэе][scс][scс]?[eэе][nн][csс][eэе]?|[сsc][оо][сsc][иi1]|[сsc][oо][сsc][аa]";
const ZERO_DIVISION_ERROR: &str = "thread 'main' panicked at 'attempt to divide by zero'";
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
    let users_to_fuck = vec![evengining_user, test_user];
    let bad_words_re = Regex::new(BAD_WORDS_RE).unwrap();

    #[async]
    for update in api.stream() {
        if let UpdateKind::Message(message) = update.kind {
            let msg = message.clone();
            if let MessageKind::Text { data, entities } = message.kind {
                let data = data.to_lowercase();
                let reply =
                    if users_to_fuck.contains(&message.from.id) && bad_words_re.is_match(&data) {
                        Some(msg.text_reply(POSHEL_NAHUY))
                    } else if data.contains(ESSENCE_IN) {
                        Some(reply_to_message(msg, ESSENCE_OUT))
                    } else if entities
                        .iter()
                        .find(|e| {
                            if e.kind == MessageEntityKind::BotCommand && e.length == 2 {
                                data.chars()
                                    .skip(e.offset as usize + 1)
                                    .take(e.length as usize)
                                    .next()
                                    .map(|ch| ch == '0')
                                    .unwrap_or(false)
                            } else {
                                false
                            }
                        })
                        .is_some()
                    {
                        Some(reply_to_message(msg, ZERO_DIVISION_ERROR))
                    } else {
                        None
                    };
                if let Some(reply) = reply {
                    if let Err(err) = await!(api.send(reply)) {
                        println!("Failed to send message: {}", err);
                    }
                }
            }
        }
    }
    Ok(())
}

fn reply_to_message(msg: Message, text: &str) -> SendMessage {
    if let Some(reply_to) = msg.reply_to_message {
        reply_to.text_reply(text)
    } else {
        msg.text_reply(text)
    }
}

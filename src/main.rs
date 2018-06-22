#![feature(proc_macro, generators)]
extern crate dotenv;
extern crate futures_await as futures;
extern crate futures_retry;
extern crate regex;
extern crate telegram_bot;
extern crate tokio_core;

use dotenv::dotenv;
use futures::prelude::*;
use futures_retry::{RetryPolicy, StreamRetryExt};
use regex::Regex;
use std::env;
use telegram_bot::prelude::*;
use telegram_bot::{Api, Message, MessageEntityKind, MessageKind, SendMessage, UpdateKind, UserId};
use tokio_core::reactor::Core;

const HELLO: &str = "https://coub.com/view/19ffik";
const BAD_WORDS_RE: &str = "[сsc][уuy][тt][ьиі1]?|[pпp][оаao][нn][иiыy1][мm][аa]|[pрr][оo][zsз][yuу][mм]|[uуаy][nн][дd][eе][рr]?[scс][tт][aэе][nн][dдт]?|[eэе][scс][scс]?[eэе][nн][csс][eэе]?|[сsc][оо][сsc][иi1]|[сsc][oо][сsc][аa]";
const ZERO_DIVISION_ERROR: &str = "thread 'main' panicked at 'attempt to divide by zero'";
const POSHEL_NAHUY: &str = "POSHEL NAHUY";
const SENSE_IN: &str = "не понимаешь сути";
const SENSE_OUT: &str = "http://telegra.ph/TY-NE-PONIMAESH-SUTI-06-22";
const PERF_IN: &str = "перформанс|перфоманс|perfomance|performance|производительность";
const PERF_OUT: &str = "http://telegra.ph/PERFORMANCE-06-22";

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
    let perf_re = Regex::new(PERF_IN).unwrap();

    #[async]
    for update in api.stream().retry(handle_update_error) {
        if let UpdateKind::Message(message) = update.kind {
            let msg = message.clone();
            match message.kind {
                MessageKind::Text { data, entities } => {
                    let data = data.to_lowercase();
                    let reply = if users_to_fuck.contains(&message.from.id)
                        && bad_words_re.is_match(&data)
                    {
                        Some(msg.text_reply(POSHEL_NAHUY))
                    } else if data.contains(SENSE_IN) {
                        Some(reply_to_message(msg, SENSE_OUT))
                    } else if perf_re.is_match(&data) {
                        Some(reply_to_message(msg, PERF_OUT))
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
                            println!("Failed to send message: {:?}", err);
                        }
                    }
                }
                MessageKind::NewChatMembers { .. } => {
                    if let Err(err) = await!(api.send(msg.text_reply(HELLO))) {
                        println!("Failed to send message: {:?}", err);
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn handle_update_error(err: telegram_bot::Error) -> RetryPolicy<telegram_bot::Error> {
    println!("An error has occurred while getting update: {:?}", err);
    RetryPolicy::Repeat
}

fn reply_to_message(msg: Message, text: &str) -> SendMessage {
    if let Some(reply_to) = msg.reply_to_message {
        reply_to.text_reply(text)
    } else {
        msg.text_reply(text)
    }
}

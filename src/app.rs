use futures::prelude::*;
use futures_retry::{RetryPolicy, StreamRetryExt};
use regex::Regex;
use std::{error::Error, fmt, io::Error as IoError};
use telegram_bot::{
    prelude::*, Api, Error as TelegramError, Message, MessageEntityKind, MessageKind, SendMessage,
    UpdateKind, UserId,
};
use tokio_core::reactor::Core;

pub type AppResult<T> = Result<T, AppError>;

pub struct App {
    api: Api,
    core: Core,
}

impl App {
    pub fn new(token: &str) -> AppResult<App> {
        let core = Core::new()?;
        let api = Api::configure(token).build(core.handle())?;
        Ok(App {
            api: api,
            core: core,
        })
    }

    pub fn run(mut self) -> AppResult<()> {
        self.core.run(handle_updates(self.api))?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct AppError {
    description: String,
}

impl From<IoError> for AppError {
    fn from(err: IoError) -> AppError {
        AppError {
            description: err.to_string(),
        }
    }
}

impl From<TelegramError> for AppError {
    fn from(err: TelegramError) -> AppError {
        AppError {
            description: err.to_string(),
        }
    }
}

impl Error for AppError {
    fn description(&self) -> &str {
        &self.description
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{}", self.description)
    }
}

const HELLO: &str = "https://coub.com/view/19ffik";
const BAD_WORDS_RE: &str = "[сsc][уuy][тt][ьиі1]?|[pпp][оаao][нn][иiыy1][мm][аa]|[pрr][оo][zsз][yuу][mм]|[uуаy][nн][дd][eе][рr]?[scс][tт][aэе][nн][dдт]?|[eэе][scс][scс]?[eэе][nн][csс][eэе]?|[сsc][оо][сsc][иi1]|[сsc][oо][сsc][аa]";
const ZERO_DIVISION_ERROR: &str = "thread 'main' panicked at 'attempt to divide by zero'";
const POSHEL_NAHUY: &str = "POSHEL NAHUY";
const SENSE_IN: &str = "не понимаешь сути";
const SENSE_OUT: &str = "http://telegra.ph/TY-NE-PONIMAESH-SUTI-06-22";
const PERF_IN: &str = "перформанс|перфоманс|perfomance|performance|производительность";
const PERF_OUT: &str = "http://telegra.ph/PERFORMANCE-06-22";

#[async]
fn handle_updates(api: Api) -> Result<(), TelegramError> {
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

fn handle_update_error(err: TelegramError) -> RetryPolicy<TelegramError> {
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

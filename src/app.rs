use rules::Rules;
use teleborg::objects::{Chat, Message, Update, User};
use teleborg::{Bot, Command, Dispatcher, Updater};

pub fn run<S: Into<String>>(token: S, rules: Rules) {
    let mut dispatcher = Dispatcher::new();
    dispatcher.add_command_handler("userid", handle_user_id, false);
    dispatcher.add_message_handler(MessageHandler { rules });
    Updater::start(Some(token.into()), None, None, None, dispatcher);
}

fn handle_user_id(bot: &Bot, update: Update, _: Option<Vec<&str>>) {
    if let Some(Message {
        reply_to_message:
            Some(box Message {
                from: Some(User { id, .. }),
                ..
            }),
        ..
    }) = update.message
    {
        if let Err(err) = bot.reply_to_message(&update, &format!("ID: {}", id)) {
            // TODO: use a logging crate
            println!("FAILED TO SEND A MESSAGE: {:?}", err);
        }
    }
}

struct MessageHandler {
    rules: Rules,
}

impl Command for MessageHandler {
    fn execute(&mut self, bot: &Bot, update: Update, _: Option<Vec<&str>>) {
        let reply = match &update.message {
            Some(Message {
                message_id,
                new_chat_member: Some(_),
                chat: Chat { id: chat_id, .. },
                ..
            }) => self.rules
                .find_new_chat_member()
                .map(|text| (chat_id, message_id, text)),
            Some(Message {
                message_id,
                text: Some(text),
                from: Some(User { id: user_id, .. }),
                chat: Chat { id: chat_id, .. },
                reply_to_message,
                ..
            }) => {
                if self.rules.has_user(user_id) {
                    self.rules
                        .find_for_user(user_id, text)
                        .map(|text| (chat_id, message_id, text))
                } else {
                    self.rules.find_any(text).map(|text| {
                        (
                            chat_id,
                            match reply_to_message {
                                Some(r) => &r.message_id,
                                None => message_id,
                            },
                            text,
                        )
                    })
                }
            }
            _ => None,
        };
        if let Some((chat_id, reply_to_id, text)) = reply {
            if let Err(err) =
                bot.send_message(chat_id, text, None, None, None, Some(reply_to_id), None)
            {
                // TODO: use a logging crate
                println!("FAILED TO SEND A MESSAGE: {:?}", err);
            }
        }
    }
}

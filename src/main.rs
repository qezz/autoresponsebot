extern crate autoresponsebot;
extern crate dotenv;

use autoresponsebot::{load_rules, run};
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let token = env::var("AUTORESPONSEBOT_TOKEN").expect("Can not to get token");
    let rules_path = env::var("AUTORESPONSEBOT_RULES").expect("Can not to get rules path");
    let rules = load_rules(rules_path).expect("Failed to load rules");
    run(token, rules);
}

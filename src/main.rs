extern crate autoresponsebot;
extern crate dotenv;
use dotenv::dotenv;

use autoresponsebot::App;
use std::env;

fn main() {
    dotenv().ok();
    let token = env::var("AUTORESPONSEBOT_TOKEN").expect("Can not to get token");
    let app = App::new(&token).expect("Failed to create app");
    app.run().expect("Run failed");
}

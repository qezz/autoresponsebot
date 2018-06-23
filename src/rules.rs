use rand::{thread_rng, Rng};
use regex::{Error as RegexError, Regex};
use serde_json::{from_str, Error as JsonError};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Error as IoError, Read};
use std::num::ParseIntError;
use std::path::Path;

const ALL_USERS: &str = "all";
const NEW_CHAT_MEMBER: &str = "new_chat_member";

pub type RulesResult<T> = Result<T, RulesError>;

type RawRules = HashMap<String, Vec<RawRule>>;

type Out = Vec<String>;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum RawRule {
    Matches { matches: String, out: Out },
    Contains { contains: String, out: Out },
    Equals { equals: String, out: Out },
    Any(String),
}

#[derive(Debug)]
struct Rule {
    input: Input,
    output: Vec<String>,
}

impl Rule {
    fn matches(&self, text: &str) -> bool {
        match self.input {
            Input::Re(ref regex) => regex.is_match(text),
            Input::Contains(ref s) => text.contains(s),
            Input::Equals(ref s) => text == s,
        }
    }

    fn random_output(&self) -> Option<&String> {
        let mut rng = thread_rng();
        rng.choose(&self.output)
    }
}

#[derive(Debug)]
enum Input {
    Re(Regex),
    Contains(String),
    Equals(String),
}

type UserId = i64;

#[derive(Default, Debug)]
pub struct Rules {
    all: Vec<Rule>,
    new_chat_member: Vec<String>,
    users: HashMap<UserId, Vec<Rule>>,
}

impl Rules {
    pub fn has_user(&self, user_id: &UserId) -> bool {
        self.users.contains_key(user_id)
    }

    pub fn find_for_user(&self, user_id: &UserId, text: &str) -> Option<&String> {
        if let Some(rules) = self.users.get(user_id) {
            rules
                .iter()
                .find(|rule| rule.matches(text))
                .and_then(|rule| rule.random_output())
        } else {
            None
        }
    }

    pub fn find_any(&self, text: &str) -> Option<&String> {
        self.all
            .iter()
            .find(|rule| rule.matches(text))
            .and_then(|rule| rule.random_output())
    }

    pub fn find_new_chat_member(&self) -> Option<&String> {
        let mut rng = thread_rng();
        rng.choose(&self.new_chat_member)
    }
}

pub fn load_from_file<P: AsRef<Path>>(path: P) -> RulesResult<Rules> {
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let mut rules = Rules::default();
    for (k, v) in from_str::<RawRules>(&buf)? {
        if k == NEW_CHAT_MEMBER {
            rules
                .new_chat_member
                .extend(convert_new_chat_member_rules(v)?);
        } else {
            let v = convert_text_rules(v)?;
            if k == ALL_USERS {
                rules.all.extend(v);
            } else {
                let user_id = k.parse::<UserId>()?;
                rules.users.insert(user_id, v);
            }
        }
    }
    Ok(rules)
}

fn convert_new_chat_member_rules(raw: Vec<RawRule>) -> RulesResult<Vec<String>> {
    let mut result = Vec::with_capacity(raw.len());
    for item in raw {
        if let RawRule::Any(text) = item {
            result.push(text)
        } else {
            return Err(RulesError::UnexpectedRule);
        }
    }
    Ok(result)
}

fn convert_text_rules(raw: Vec<RawRule>) -> RulesResult<Vec<Rule>> {
    let mut result = Vec::with_capacity(raw.len());
    for item in raw {
        result.push(match item {
            RawRule::Matches { matches, out } => Rule {
                input: Input::Re(Regex::new(&matches)?),
                output: out,
            },
            RawRule::Contains { contains, out } => Rule {
                input: Input::Contains(contains),
                output: out,
            },
            RawRule::Equals { equals, out } => Rule {
                input: Input::Equals(equals),
                output: out,
            },
            RawRule::Any(_) => return Err(RulesError::UnexpectedRule),
        });
    }
    Ok(result)
}

#[derive(Debug)]
pub enum RulesError {
    Io(IoError),
    Json(JsonError),
    ParseInt(ParseIntError),
    Regex(RegexError),
    UnexpectedRule,
}

impl From<IoError> for RulesError {
    fn from(err: IoError) -> RulesError {
        RulesError::Io(err)
    }
}

impl From<JsonError> for RulesError {
    fn from(err: JsonError) -> RulesError {
        RulesError::Json(err)
    }
}

impl From<ParseIntError> for RulesError {
    fn from(err: ParseIntError) -> RulesError {
        RulesError::ParseInt(err)
    }
}

impl From<RegexError> for RulesError {
    fn from(err: RegexError) -> RulesError {
        RulesError::Regex(err)
    }
}

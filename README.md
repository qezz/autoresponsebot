# Autoresponsebot

## Environment variables

```
# Telegram bot token
AUTORESPONSEBOT_TOKEN=bottoken
# Path to rules file
AUTORESPONSEBOT_RULES=./rules.json
```

## Rules


```
{
    "telegram-user-id": {
        {
            "matches": "[a-z0-9]",
            "out": [
                "message matches to a regex"
            ]
        }
    },
    "all": {
        {
            "contains": "text",
            "out": [
                "message contains a text"
            ]
        },
        {
            "equals": "text",
            "out": ["equals to text"]
        },
    },
    "new_chat_member": {
        "list of messages",
        "for new_chat_memeber event"
    }
}
```

Reply to a message with `/userid` command in order to get a user ID.

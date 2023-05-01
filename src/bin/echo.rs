use std::io::BufRead;
use std::io::stdin;

use serde::{Deserialize, Serialize};

// {
//     "type":     "init",
//     "msg_id":   1,
//     "node_id":  "n3",
//     "node_ids": ["n1", "n2", "n3"]
//   }

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    src: String,
    dest: String,
    body: Body,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Body {
    Init {
        msg_id: u64,
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk {
        in_reply_to: u64,
    },
    Echo {
        msg_id: u64,
        echo: String,
    },
    EchoOk {
        msg_id: u64,
        in_reply_to: u64,
        echo: String,
    },
}

fn handle_message(
    message: Message,
) -> anyhow::Result<()> {
    match message.body {
        Body::Init { msg_id, .. } => {
            let response = Message {
                src: message.dest,
                dest: message.src,
                body: Body::InitOk {
                    in_reply_to: msg_id,
                },
            };

            println!("{}", serde_json::to_string(&response)?);
        },
        Body::Echo { msg_id, echo } => {
            let response = Message {
                src: message.dest,
                dest: message.src,
                body: Body::EchoOk {
                    msg_id,
                    in_reply_to: msg_id,
                    echo,
                },
            };

            println!("{}", serde_json::to_string(&response)?);
        },
        Body::EchoOk { .. } | Body::InitOk { .. }=> {},
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let lines = stdin().lock().lines();

    for line in lines {
        handle_message(serde_json::from_str::<Message>(&line?)?)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let s = r#"{
            "src": "n1",
            "dest": "n2",
            "body": {
                "type": "init",
                "msg_id": 1,
                "node_id": "n3",
                "node_ids": ["n1", "n2", "n3"]
            }
        }"#;

        let message = serde_json::from_str::<Message>(s).unwrap();
        assert!(matches!(message.body, Body::Init { .. }));
    }

    #[test]
    fn test_init_ok() {
        let s = r#"{
            "src": "n1",
            "dest": "n2",
            "body": {
                "type": "init_ok",
                "in_reply_to": 1
            }
        }"#;

        let message = serde_json::from_str::<Message>(s).unwrap();
        assert!(matches!(message.body, Body::InitOk { .. }));
    }
}
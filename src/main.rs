use reqwest::Client;
use serde_json::json;
use std::{env, fmt};
use sysinfo::System;
use time::{format_description::well_known::Rfc2822, OffsetDateTime};

#[derive(Debug, PartialEq)]
enum Action {
    PowerUp,
    Shutdown,
    Schedule,
    Test,
    Reboot,
    Unknown,
}

impl Action {
    fn from_str(action: &str) -> Self {
        match action {
            "powerup" | "up" => Action::PowerUp,
            "shutdown" | "down" => Action::Shutdown,
            "schedule" => Action::Schedule,
            "test" | "t" => Action::Test,
            "reboot" | "r" => Action::Reboot,
            _ => Action::Unknown,
        }
    }

    fn to_string(&self) -> String {
        match self {
            Action::PowerUp => String::from("System Powerup"),
            Action::Shutdown => String::from("System Shutdown"),
            Action::Schedule => String::from("System Shutdown Scheduled"),
            Action::Test => String::from("Test"),
            Action::Reboot => String::from("System Reboot"),
            Action::Unknown => String::from("Unknown"),
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

struct Message {
    action: Action,
    computer_name: String,
    os_info: String,
    current_time: String,
    scheduled_time: Option<OffsetDateTime>,
}

impl Message {
    fn from_action(action: Action) -> Self {
        let action = action;
        let computer_name = get_computer_name();
        let os_info = get_os_info();
        let current_time = get_current_time();
        let scheduled_time = None;

        Message {
            action,
            computer_name,
            os_info,
            current_time,
            scheduled_time,
        }
    }

    fn create_message(&self) -> String {
        match self.action {
            Action::Schedule => {
                let scheduled_time = self
                    .scheduled_time
                    .map(|time| get_readable_time(time))
                    .unwrap_or_else(|| "Unknown".to_string());

                format!(
                    r#"{} at {}:
```markdown
Computer     | `{}`
Running      | `{}`
Time         | `{}`
```"#,
                    self.action,
                    scheduled_time,
                    self.computer_name,
                    self.os_info,
                    self.current_time
                )
            }
            _ => format!(
                r#"{} Alert:
```markdown
Computer     | `{}`
Running      | `{}`
Time         | `{}`
```"#,
                self.action, self.computer_name, self.os_info, self.current_time
            ),
        }
    }
}

fn get_readable_time(time: OffsetDateTime) -> String {
    time.format(&Rfc2822)
        .ok()
        .unwrap_or_else(|| "Unknown".to_string())
        .to_string()
}

fn get_current_time() -> String {
    let current_time_local = OffsetDateTime::now_local();

    return match current_time_local {
        Ok(time) => get_readable_time(time),
        Err(_) => "Unknown".to_string(),
    };
}

fn get_system_name() -> String {
    System::name().unwrap_or("Unknown".to_string())
}

fn get_computer_name() -> String {
    System::host_name().unwrap_or("Unknown".to_string())
}

fn get_os_version() -> String {
    System::os_version().unwrap_or("Unknown".to_string())
}

fn get_os_info() -> String {
    let system_name = get_system_name();
    let os_version = get_os_version();
    format!("{} {}", system_name, os_version)
}

async fn send_discord_message(webhook_url: &str, message: Message) -> Result<(), reqwest::Error> {

    let json_payload = json!({
        "content": message.create_message()
    });

    let client = Client::new();
    let res = client
        .post(webhook_url)
        .header("Content-Type", "application/json")
        .body(json_payload.to_string())
        .send()
        .await?;

    #[cfg(debug_assertions)]
    {
        println!("{:?}", res);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok(); // Load environment variables from .env file
    let webhook_url = env::var("DISCORD_WEBHOOK_URL").expect("DISCORD_WEBHOOK_URL not set");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide an action as a command-line argument.");
        std::process::exit(1);
    }

    let action_str = &args[1];
    let action = Action::from_str(action_str);

    let message = Message::from_action(action);

    send_discord_message(&webhook_url, message).await?;

    Ok(())
}

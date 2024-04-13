use chrono::{DateTime, Duration, Local, TimeDelta};
use reqwest::Client;
use serde_json::json;
use std::{env, fmt};
use sysinfo::System;

#[derive(Debug, PartialEq)]
enum Action {
    PowerUp,
    Shutdown,
    Schedule(Option<TimeDelta>),
    Test,
    Reboot,
    Unknown,
}

impl Action {
    fn from_str(action: &str, time_delta: Option<TimeDelta>) -> Self {
        match action {
            "powerup" | "up" => Action::PowerUp,
            "shutdown" | "down" => Action::Shutdown,
            "schedule" => Action::Schedule(time_delta),
            "test" | "t" => Action::Test,
            "reboot" | "r" => Action::Reboot,
            _ => Action::Unknown,
        }
    }

    fn from_cli() -> Result<Action, &'static str> {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            return Err("Please provide an action as a command-line argument.");
        }

        let action_str = &args[1];
        let amount_of_time: Option<TimeDelta> = if args.len() >= 3 {
            Duration::try_minutes(args[2].parse().unwrap())
        } else {
            None
        };

        if action_str == "schedule" && amount_of_time.is_none() {
            return Err("Please provide a time in minutes when scheduling.");
        }

        let action = Action::from_str(action_str, amount_of_time);

        match action {
            Action::Unknown => Err("Invalid action provided."),
            _ => Ok(action),
        }
    }

    fn to_string(&self) -> String {
        match self {
            Action::PowerUp => String::from("System Powerup"),

            Action::Shutdown => String::from("System Shutdown"),

            Action::Schedule(time_delta) => {
                if time_delta.is_none() {
                    panic!("No time provided.");
                }
                String::from(format!(
                    "System Shutdown Scheduled at {}",
                    Time::get_formatted_time(Time::get_scheduled_time(
                        time_delta.unwrap().num_minutes()
                    ))
                ))
            }

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

struct Time {}

impl Time {
    fn get_formatted_time(time: DateTime<Local>) -> String {
        time.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    fn get_scheduled_time(offset: i64) -> DateTime<Local> {
        Local::now() + Duration::minutes(offset)
    }

    fn get_current_time() -> DateTime<Local> {
        Local::now()
    }
}

struct SystemInfo {
    computer_name: String,
    _system_name: String,
    os_info: String,
    _os_version: String,
}

impl SystemInfo {
    fn new() -> Self {
        let computer_name = SystemInfo::get_computer_name();
        let _system_name = SystemInfo::get_system_name();
        let _os_version = SystemInfo::get_os_version();
        let os_info = SystemInfo::get_os_info();

        SystemInfo {
            computer_name,
            _system_name,
            _os_version,
            os_info,
        }
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
        let system_name = SystemInfo::get_system_name();
        let os_version = SystemInfo::get_os_version();
        format!("{} {}", system_name, os_version)
    }
}

struct Message {
    action: Action,
    system: SystemInfo,
    current_time: DateTime<Local>,
}

impl Message {
    fn from_action(action: Action) -> Self {
        let action = action;
        let system = SystemInfo::new();
        let current_time = Time::get_current_time();

        Message {
            action,
            system,
            current_time,
        }
    }

    fn create_message(&self) -> String {
        let title = format!("{}:", self.action);

        let body = format!(
            r#"```markdown
Computer     | `{}`
Running      | `{}`
Alert Time   | `{}`
```"#,
            self.system.computer_name,
            self.system.os_info,
            Time::get_formatted_time(self.current_time)
        );
        return format!("{}\n{}", title, body);
    }
}

struct Sender {
    webhook_url: String,
}

impl Sender {
    fn from_env() -> Self {
        // Load environment variables from .env file
        dotenv::dotenv().ok();

        let webhook_url = env::var("DISCORD_WEBHOOK_URL").expect("DISCORD_WEBHOOK_URL not set");

        Sender { webhook_url }
    }

    async fn send_discord_message(&self, message: Message) -> Result<(), reqwest::Error> {
        let json_payload = json!({
            "content": message.create_message()
        });

        let client = Client::new();
        let res = client
            .post(&self.webhook_url)
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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sender = Sender::from_env();

    let action = Action::from_cli()?;

    let message = Message::from_action(action);

    sender.send_discord_message(message).await?;

    Ok(())
}

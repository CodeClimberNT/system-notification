use reqwest::Client;
use serde_json::json;
use std::env;
// use std::time::{SystemTime, UNIX_EPOCH};
use sysinfo::System;
use time::{OffsetDateTime, format_description::well_known::Rfc2822};

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

    fn to_string(&self) -> &str {
        match self {
            Action::PowerUp => "System Powerup",
            Action::Shutdown => "System Shutdown",
            Action::Schedule => "System Shutdown Scheduled",
            Action::Test => "Test",
            Action::Reboot => "System Reboot",
            Action::Unknown => "Unknown Action",
        }
    }
}

fn get_current_time() -> String {
    let current_time_local = OffsetDateTime::now_local();

    return match current_time_local{
        Ok(time) => time.format(&Rfc2822).ok().unwrap_or("Unknown".to_string()).to_string(),
        Err(_) => "Unknown".to_string()
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
    let system_name = get_system_name();
    let os_version = get_os_version();
    format!("{} {}", system_name, os_version)
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

    let computer_name = get_computer_name();
    let os_info = get_os_info();
    let current_time = get_current_time();

    let action_title = action.to_string();

    let message = format!(
        r#"{} Alert:
```markdown
Computer     | `{}`
Running      | `{}`
Time         | `{}`
```"#,
        action_title, computer_name, os_info, current_time
    );

    let json_payload = json!({
        "content": message
    });

    let client = Client::new();
    let res = client
        .post(&webhook_url)
        .header("Content-Type", "application/json")
        .body(json_payload.to_string())
        .send()
        .await?;

    println!("{:?}", res);

    Ok(())
}

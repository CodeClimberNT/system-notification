use chrono::{DateTime, Local, TimeDelta};
use dotenvy::dotenv;
use reqwest::Client;
use serde_json::json;
use std::{env, fmt};
use sysinfo::System;

// ==================== Action Definition ====================

type ParseResult = Result<Action, Box<dyn std::error::Error>>;

/// A declarative definition of an action's CLI contract and how to build it.
struct ActionDef {
    name: &'static str,
    aliases: &'static [&'static str],
    expected_args: usize,
    /// A function pointer that takes the arguments slice and returns the Action
    parser: fn(&[String]) -> ParseResult,
}

// THE SINGLE SOURCE OF TRUTH.
// Adding a new action means just adding one block here and a variant to the enum.
const ACTION_DEFS: &[ActionDef] = &[
    ActionDef {
        name: "powerup",
        aliases: &["powerup", "up"],
        expected_args: 0,
        parser: |_| Ok(Action::PowerUp),
    },
    ActionDef {
        name: "shutdown",
        aliases: &["shutdown", "down"],
        expected_args: 0,
        parser: |_| Ok(Action::Shutdown),
    },
    ActionDef {
        name: "schedule",
        aliases: &["schedule"],
        expected_args: 1,
        parser: |args| {
            let minutes: i64 = args[0].parse().map_err(|_| "invalid number of minutes")?;
            let td = TimeDelta::try_minutes(minutes).ok_or("time delta out of range")?;
            Ok(Action::Schedule(td))
        },
    },
    ActionDef {
        name: "test",
        aliases: &["test", "t"],
        expected_args: 0,
        parser: |_| Ok(Action::Test),
    },
    ActionDef {
        name: "reboot",
        aliases: &["reboot", "r"],
        expected_args: 0,
        parser: |_| Ok(Action::Reboot),
    },
];

// ==================== Action ====================

#[derive(Debug, PartialEq)]
enum Action {
    PowerUp,
    Shutdown,
    Schedule(TimeDelta),
    Test,
    Reboot,
}

impl Action {
    fn from_cli() -> Result<Self, Box<dyn std::error::Error>> {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            return Err("Please provide an action as a command-line argument.");
        }

        let action_str = args[1].as_str();
        let provided_args = &args[2..]; // The rest of the arguments

        // 1. Find the definition dynamically (no string match arms!)
        let def = ACTION_DEFS
            .iter()
            .find(|def| def.aliases.contains(&action_str))
            .ok_or("invalid action provided")?;

        // 2. Automatically enforce argument counts based on the definition
        if provided_args.len() != def.expected_args {
            return Err(format!(
                "'{}' requires exactly {} argument(s)",
                def.name, def.expected_args
            )
            .into());
        }

        // 3. Invoke the function pointer to build the Action
        (def.parser)(provided_args)
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The compiler enforces that we cover all variants.
        // We write directly to `f` (the output stream) without allocating a temporary String!
        match self {
            Self::PowerUp => write!(f, "System Power Up"),
            Self::Shutdown => write!(f, "System Shutdown"),
            Self::Schedule(td) => write!(
                f,
                "System Shutdown Scheduled at {}",
                format_time(get_scheduled_time(td.num_minutes()))
            ),
            Self::Test => write!(f, "Test"),
            Self::Reboot => write!(f, "System Reboot"),
        }
    }
}

// ==================== Time helpers ====================

fn format_time(time: DateTime<Local>) -> String {
    time.format("%Y-%m-%d %H:%M:%S").to_string()
}

fn get_scheduled_time(offset: i64) -> DateTime<Local> {
    Local::now() + TimeDelta::minutes(offset)
}

fn get_current_time() -> DateTime<Local> {
    Local::now()
}

// ==================== SystemInfo ====================

struct SystemInfo {
    computer_name: String,
    os_info: String,
}

impl SystemInfo {
    fn new() -> Self {
        SystemInfo {
            computer_name: SystemInfo::get_computer_name(),
            os_info: SystemInfo::get_os_info(),
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
        format!("{system_name} {os_version}")
    }
}

// ==================== Message ====================

struct Message {
    action: Action,
    system: SystemInfo,
    current_time: DateTime<Local>,
}

impl Message {
    fn from_action(action: Action) -> Self {
        Self {
            action,
            system: SystemInfo::new(),
            current_time: get_current_time(),
        }
    }

    fn create_message(&self) -> String {
        let title = format!("{}:", self.action);

        let body = format!(
            r"```markdown
Computer     | `{}`
Running      | `{}`
Alert Time   | `{}`
```",
            self.system.computer_name,
            self.system.os_info,
            format_time(self.current_time)
        );
        format!("{title}\n{body}")
    }
}

// ==================== Sender ====================

struct Sender {
    client: Client,
    webhook_url: String,
}

impl Sender {
    fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let webhook_url =
            env::var("DISCORD_WEBHOOK_URL").map_err(|_| "DISCORD_WEBHOOK_URL not set")?;

        Ok(Self {
            client: Client::new(),
            webhook_url,
        })
    }

    async fn send(&self, message: Message) -> Result<(), Box<dyn std::error::Error>> {
        let json_payload = json!({
            "content": message.create_message()
        });

        let res = self
            .client
            .post(&self.webhook_url)
            .json(&json_payload)
            .send()
            .await?;

        #[cfg(debug_assertions)]
        {
            println!("{res:?}");
        }

        res.error_for_status()?;

        Ok(())
    }
}

// ==================== main ====================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let sender = Sender::from_env()?;
    let action = Action::from_cli()?;
    let message = Message::from_action(action);

    sender.send(message).await?;

    Ok(())
}

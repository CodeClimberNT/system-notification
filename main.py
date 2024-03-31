import os
import sys
import asyncio
import socket
import platform
from datetime import datetime, timedelta
from dotenv import load_dotenv
from telegram import Bot

# Load environment variables from .env file
load_dotenv()

# Get Telegram bot token and chat ID from environment variables
bot_token = os.getenv("TELEGRAM_BOT_TOKEN")
chat_id = os.getenv("TELEGRAM_CHAT_ID")

valid_actions = ["powerup", "up", "shutdown", "down", "schedule", "test", "t"]

def print_usage_and_exit():
    print("Usage: python script.py <action>")
    print(f"Invalid action. Valid actions: {', '.join(valid_actions)}")
    sys.exit(1)


async def send_telegram_message(message):
    bot = Bot(token=bot_token)
    await bot.send_message(chat_id=chat_id, text=message, parse_mode="Markdown")


def main():
    if len(sys.argv) < 2 or len(sys.argv) > 3:
        print_usage_and_exit()

    action = sys.argv[1]
    
    if action not in valid_actions:
        print_usage_and_exit()


    # Get the current date and time
    current_time = datetime.now().strftime("%Y-%m-%d %H:%M:%S")

    # Get the computer name
    computer_name = socket.gethostname()

    # Get the operating system version
    os_version = platform.platform()

        # Define action titles
    action_titles = {
        "powerup": "System Powerup",
        "up": "System Powerup",
        "shutdown": "System Shutdown",
        "down": "System Shutdown",
        "schedule": "System Shutdown Scheduled",
        "test": "Test",
        "t": "Test"
    }

    # Concatenate action title to message body
    action_title = action_titles.get(action)


    # Concatenate action title to message body
    action_title = action_titles.get(action)
    if action == "schedule":
        if len(sys.argv) != 3:
            print("Usage: python script.py schedule <time_in_minutes>")
            sys.exit(1)
        try:
            minutes_offset = int(sys.argv[2])
        except ValueError:
            print("Time must be an integer representing minutes")
            sys.exit(1)
        schedule_time = datetime.now() + timedelta(minutes=minutes_offset)
        message_body = f" at {schedule_time.strftime('%Y-%m-%d %H:%M:%S')}:"
    else:
        message_body = " Alert:"

    # Create the message
    message = f"""\
{action_title}{message_body}
```markdown
Computer      | `{computer_name}`
Running       | `{os_version}`
Time          | `{current_time}`
```"""

    # Send the message
    asyncio.run(send_telegram_message(message))


if __name__ == "__main__":
    main()

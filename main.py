import os
import sys
import asyncio
import socket
import platform
from datetime import datetime
from dotenv import load_dotenv
from telegram import Bot

# Load environment variables from .env file
load_dotenv()

# Get Telegram bot token and chat ID from environment variables
bot_token = os.getenv("TELEGRAM_BOT_TOKEN")
chat_id = os.getenv("TELEGRAM_CHAT_ID")


async def send_telegram_message(message):
    bot = Bot(token=bot_token)
    await bot.send_message(chat_id=chat_id, text=message, parse_mode="Markdown")


def main():
    if len(sys.argv) != 2:
        print("Usage: python script.py <action>")
        print("Valid actions: powerup, shutdown")
        sys.exit(1)
    
    action = sys.argv[1]

    # Get the current date and time
    current_time = datetime.now().strftime("%Y-%m-%d %H:%M:%S")

    # Get the computer name
    computer_name = socket.gethostname()

    # Get the operating system version
    os_version = platform.platform()

    if (action == "powerup" or action == "p" or action == "up"):
            # Create the message as a table-like structure
        message = f"""\
System Powerup Alert:
```markdown
*Parameter*   | *Value*
--------------|-------------------------
Computer      | `{computer_name}`
Running       | `{os_version}`
Powered on at | `{current_time}`
```"""
    elif (action == "shutdown" or action == "s" or action == "down"):
        message = f"""\
System Shutdown Alert:
```markdown
*Parameter*   | *Value*
--------------|-------------------------
Computer      | `{computer_name}`
Running       | `{os_version}`
Powered on at | `{current_time}`
```"""
    else:
        print("Invalid action. Valid actions: powerup, shutdown")
        sys.exit(1)

    # Send the message
    asyncio.run(send_telegram_message(message))


if __name__ == "__main__":
    main()

[![Test](https://github.com/CodeClimberNT/system-notification/actions/workflows/test.yml/badge.svg)](https://github.com/CodeClimberNT/system-notification/actions/workflows/test.yml)

# System Notification
Automating System Notification using rust

## Usage
Very easy to use, launch the program with the terminal with an arguments to send a notification

| Args            | Description                                                                    |
|-----------------|--------------------------------------------------------------------------------|
| powerup (up)    | send a notification that the system was powered on                             |
| shutdown (down) | send a notification that the system was powered off                            |
| schedule X      | Send a notification that the system scheduled will be powered off in X minutes |
| reboot          | Send a notification that the system started a reboot event                     |
| test            | Send a test alert                                                              |

⚠️ REMEMBER ⚠️ schedule require also a value to tell when the schedule will happen!

## Installation
For now the program send a discord message using webhook. That simply means that other than an url you don't need to do anything.

To start, go to the discord channel where you want the alert to arrive. Go into its option and then go into the webhook section. 
Here create a new webhook, you can do wathever you want with it, the only important part is its url.
copy it and paste it inside a file called `.env` (you need to create it) in the same folder where the system-notification program is located.

the file should follow the following structure:
`ENV_NAME=VALUE` (NO SPACE!)<br>

In this case the `ENV_NAME` is `DISCORD_WEBHOOK_URL` and `VALUE` is the url you copied, so you should have inside the `.env` file something like this:

`DISCORD_WEBHOOK_URL=https://discord.com/api/webhooks/{SOME_RANDOM_NUMBERS}/{SOME_RANDOM_LETTERS}`

### That's it!
Now launch the program from the terminal with the arguments that you like and you should recieve a nice new message in the server!

## TODO
- [ ] Better README
- [ ] ⚠️ Add other types of notification
- [ ] Implementing a multiplatform application
- [ ] Others (?)
      
## Done
- [x] Notification when system schedule to power off
- [x] Single file to launch with parameters
- [x] Rewritten in Rust
- [x] Discord Webhook
- [x] Better Code Modularity


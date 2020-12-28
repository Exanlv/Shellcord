# Shellcord
Interact with Discord using shell

## Currently supported events:
Message created

## Currently supported methods:
Create message

## Currently supported OSses:
Should work with anything unix-like as long as you can compile rust

## How it works:
The main process src/main.rs spawns child processes with `sh` running scripts to handle the events. Methods are called based on `stdout`

```SEND_MESSAGE $CHANNEL_ID pong!```
sends a message to `$CHANNEL_ID`

# Extra wayland shell event loop and their iced bindings

We want to make program with iced for layershell and sessionlock, so we made this project.

Take winit as reference a lot, to make easilier program on layershell and ext-session-lock.

This project bind `ext-session-lock` and `layershell` with the similar way of winit, which storing message and handle it in callback

## How can it do such thing?

Iced itself does not support to add extra custom system actions or extra system events, but we can do something to Message. We add some genertic type restriction to the message, and let message can be changed to events or actions during eventloop, so you need to use the macros, and add extra fields to your `Message`. And I am lazy about writing the documents, so if you do not understand how to use this crate, you can always ask questions in discord channels, open issues or make pr for us. By the way, about custom events, we have pr about it at https://github.com/iced-rs/iced/pull/2658.

And we also support popup in these shells, but since the popup/tooltip support has not landed in winit/iced, so it seems only a toy. I have consider about a design for it, but I do not have time to implement it, and I need to read the code about iced.

Always welcome pr and issues!

## Here are four main subprojects

### waycrate_xkbkeycode
[![Crates.io](https://img.shields.io/crates/v/waycrate_xkbkeycode.svg)](https://crates.io/crates/waycrate_xkbkeycode)

Take a lot of reference from winit (mainly from winit). Mainly handle the xkbcommon events.

### exwlshellev
[![Crates.io](https://img.shields.io/crates/v/exwlshellev.svg)](https://crates.io/crates/exwlshellev)

All wayland extra shell in one eventloop. It contains layershell, sessionlock and input-panel. This libraries provides full extra shell support for iced_exwlshell

### iced_exwlshell
[![Crates.io](https://img.shields.io/crates/v/iced_exwlshell.svg)](https://crates.io/crates/iced_exwlshell)

Full extra shell binding for iced

Now you can use this crate to make a shell probram, including lock, dock, and etc

#### Feature:

- support to open new layershell and support popup window.
- support ext-virtual-keyboard
- support sessionlock

### iced_wayland_subscriber
[![Crates.io](https://img.shields.io/crates/v/iced_wayland_subscriber.svg)](https://crates.io/crates/iced_wayland_subscriber)

This crate provides subscriptions for many wayland events, like ext-workspace, outputs and etc. You can use this crate to listen on wayland events.

## NOTE

From version 0.20.0, iced_layershell and iced_sessionlock will be deprecated. The related functions and macros are moved to iced_exwlshell. Please read the guidance under `docs` folder.

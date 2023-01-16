# canvas-sync

A barebones CLI tool that keeps local folders up-to-date with online
folders on Canvas.

- [Install](#install)
- [Configuration](#configuration)
- [Usage](#usage)

## Install

Installing `canvas-sync` currently requires an installation of
`cargo`. To install cargo, follow these [awesome
instructions][cargo-install]. Once you have cargo installed, you can
now install `canvas-sync` with

```sh
cargo install canvas-sync
```

## Configuration

Depending on your operating system, `canvas-sync` chooses a different
default configuration file location. To find out this location, run

```
canvas-sync config
```

If it doesn't exist, then create it and fill it in with this template:

```yaml
---
access_token: a_very_secret_value
base_path: /path/to/your/base # optional
folders:
  - url: "https://canvas.nus.edu.sg/courses/12345/files/folder/Lecture%20Notes"
    path: MA2101/lec
  - url: "https://canvas.nus.edu.sg/courses/98765/files/folder/Tutorials"
    path: MA2104/tut
```

1. `access_token` - this is what authenticates into canvas instead of
   a username and password. This can be found at your [canvas profile
   settings](https://canvas.nus.edu.sg/profile/settings) and looking
   around for the 'token' keyword. Generate a fresh one and make sure
   to save the token string to a safe location.

2. `base_path` - this is an optional parameter that will pre-pend all
   other paths in your config. Leave this out of your config if you
   want greater freedom in specifying each path. Otherwise, it's a
   nice way to shorten all your other paths.

3. `folders` - this is an array of `{ url, path }` objects. `url` points
   to the folder on canvas that you want to track. `path` points to
   the local directory on your computer that you want to be synced
   with that folder online.

### Specifying urls

In each `{ url, path }` object, is the page that each url should point to:

![canvas-demo](https://user-images.githubusercontent.com/10664455/212221239-1799d6fa-504e-4b69-9908-1235b6f4b2af.jpg)

`path` will then track the contents of this folder.

## Usage

Once you have specified your [configuration](#configuration), there
are a few commands that `canvas-sync` supports:

```sh
canvas-sync        # ping canvas servers to check if token is valid
canvas-sync fetch  # fetch updates without downloading
canvas-sync pull   # fetch and download updates
canvas-sync config # see where your config.yml is stored.
canvas-sync set-token <token>  # set your token
```

[cargo-install]: https://doc.rust-lang.org/cargo/getting-started/installation.html

# canvas-sync

A barebones CLI tool that keeps local folders up-to-date with online
folders on Canvas.

- [Configuration](#configuration)
- [Usage](#usage)

## Configuration

Make a `canvas.json` file and save it somewhere in your system.

```JSON
{
  "token": "",
  "base_path": "~/files/canvas-api",
  "maps": [
    {
      "url": "https://canvas.nus.edu.sg/courses/36736/files/folder/Tutorials",
      "path": "MA2104/tut"
    },
    {
      "url": "https://canvas.nus.edu.sg/courses/36741/files/folder/Lecture%20Notes",
      "path": "MA2108/lec"
    },
  ]
}
```

There are three key components in your config:

1. `token` - this is what authenticates into canvas instead of a
   username and password. This can be found at your [canvas profile
   settings](https://canvas.nus.edu.sg/profile/settings) and looking
   around for the 'token' keyword. Generate a fresh one and make sure
   to save the token string to a safe location.

2. `base_path` - this is an optional parameter that will pre-pend all
   other paths in your config. Leave this out of your config if you
   want greater freedom in specifying each path. Otherwise, it's a
   nice way to shorten all your other paths.

3. `maps` - this is an array of `{ url, path }` objects. `url` points
   to the folder on canvas that you want to track. `path` points to
   the local directory on your computer that you want to be synced
   with that folder online.

### Specificying urls

In each `{ url, path }` object, is the page that each url should point to:

![canvas-demo](https://user-images.githubusercontent.com/10664455/212221239-1799d6fa-504e-4b69-9908-1235b6f4b2af.jpg)

`path` will then track the contents of this folder.

## Usage

Once you have specified your [configuration](#configuration), to make
your local folders update to date with canvas, execute the command

```sh
canvas-sync /path/to/canvas.json
```

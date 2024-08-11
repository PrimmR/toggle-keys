# Toggle Keys

This is a WIP crate that provides an interface for querying the state of togglable keys (`Caps Lock`, `Num Lock`, etc.) across multiple OSes. This is intended to be used to for situations where the conventional LED indicators may not be viewable. 

## Development

This crate is very much still in development, so it currently only has limited functionality. Currently:

- Only the `Caps Lock` key can be queried
- Windows has full support
- Linux has partial support, as the user needs to have input privileges to fetch the state of the keyboard
- macOS has no support

I intend to flesh out this library somewhat by adding a wider range of keys, but I'm unlikely to improve the OS support myself.
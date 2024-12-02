# Nova

Nova is a 2D game framework written in Rust.

This is a sort of spiritual successor to [Tetra](https://github.com/17cupsofcoffee/tetra), a game engine I worked
on between 2018 and 2022. It aims to be smaller and simpler, with less global state.

**⚠️ Use at your own risk!** This framework is still very experimental, and the API is constantly in flux. No support is offered, but you are welcome to use to the code as reference or fork it for your own needs.

## Features

- `ldtk` (enabled by default): enables a module to load [ldtk](https://ldtk.io/) files.
- `static_bundled_build`: enables automatic SDL3 library building and linking. Building SDL3 can take a bit during that first build (usually 1 minute or more).

## Notes

- This framework is very heavily inspired by [FNA](https://github.com/FNA-XNA/FNA), and NoelFB's lightweight game engines ([Blah](https://github.com/NoelFB/blah) and [Foster](https://github.com/NoelFB/Foster)).
- It depends on [SDL3](https://www.libsdl.org/) for interacting with the underlying platform.

# Civilization VI mlua

Civilization VI mlua is a fork of mlua adding support for using the lua API symbols exported by Civilization VI.

`mlua` is bindings to [Lua](https://www.lua.org) programming language for Rust with a goal to provide
_safe_ (as far as it's possible), high level, easy to use, practical and flexible API.

Started as `rlua` fork, `mlua` supports Lua 5.4, 5.3, 5.2, 5.1 (including LuaJIT) and [Roblox Luau] and allows to write native Lua modules in Rust as well as use Lua in a standalone mode.

`mlua` tested on Windows/macOS/Linux including module mode in [GitHub Actions] on `x86_64` platform and cross-compilation to `aarch64` (other targets are also supported).

WebAssembly (WASM) is supported through `wasm32-unknown-emscripten` target for all Lua versions excluding JIT.

## License

This project is licensed under the [MIT license](LICENSE)

//! Contains definitions from `lualib.h`.

use std::os::raw::c_int;

use super::lua::lua_State;

pub const LUA_COLIBNAME: &str = "coroutine";
pub const LUA_TABLIBNAME: &str = "table";
pub const LUA_IOLIBNAME: &str = "io";
pub const LUA_OSLIBNAME: &str = "os";
pub const LUA_STRLIBNAME: &str = "string";
pub const LUA_MATHLIBNAME: &str = "math";
pub const LUA_DBLIBNAME: &str = "debug";
pub const LUA_LOADLIBNAME: &str = "package";

#[cfg_attr(
    all(windows, raw_dylib),
    link(name = "HavokScript_FinalRelease", kind = "raw-dylib")
)]
extern "C-unwind" {
    #[link_name = "?luaopen_base@@YAHPEAUlua_State@@@Z"]
    pub fn luaopen_base(L: *mut lua_State) -> c_int;
    #[link_name = "?luaopen_table@@YAHPEAUlua_State@@@Z"]
    pub fn luaopen_table(L: *mut lua_State) -> c_int;
    #[link_name = "?luaopen_io@@YAHPEAUlua_State@@@Z"]
    pub fn luaopen_io(L: *mut lua_State) -> c_int;
    #[link_name = "?luaopen_os@@YAHPEAUlua_State@@@Z"]
    pub fn luaopen_os(L: *mut lua_State) -> c_int;
    #[link_name = "?luaopen_string@@YAHPEAUlua_State@@@Z"]
    pub fn luaopen_string(L: *mut lua_State) -> c_int;
    #[link_name = "?luaopen_math@@YAHPEAUlua_State@@@Z"]
    pub fn luaopen_math(L: *mut lua_State) -> c_int;
    #[link_name = "?luaopen_debug@@YAHPEAUlua_State@@@Z"]
    pub fn luaopen_debug(L: *mut lua_State) -> c_int;
    #[link_name = "?luaopen_package@@YAHPEAUlua_State@@@Z"]
    pub fn luaopen_package(L: *mut lua_State) -> c_int;

    // open all builtin libraries
    #[link_name = "?luaL_openlibs@@YAXPEAUlua_State@@@Z"]
    pub fn luaL_openlibs(L: *mut lua_State);
}

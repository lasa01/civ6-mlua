//! Contains definitions from `lauxlib.h`.

use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

use super::lua::{self, lua_CFunction, lua_Integer, lua_Number, lua_State};

// Extra error code for 'luaL_load'
pub const LUA_ERRFILE: c_int = lua::LUA_ERRERR + 1;

#[repr(C)]
pub struct luaL_Reg {
    pub name: *const c_char,
    pub func: lua_CFunction,
}

#[cfg_attr(
    all(windows, raw_dylib),
    link(name = "HavokScript_FinalRelease", kind = "raw-dylib")
)]
extern "C-unwind" {
    #[link_name = "?luaL_register@@YAXPEAUlua_State@@PEBDPEBUluaL_Reg@@@Z"]
    pub fn luaL_register(L: *mut lua_State, libname: *const c_char, l: *const luaL_Reg);
    #[link_name = "?luaL_getmetafield@@YAHPEAUlua_State@@HPEBD@Z"]
    pub fn luaL_getmetafield_(L: *mut lua_State, obj: c_int, e: *const c_char) -> c_int;
    #[link_name = "?luaL_callmeta@@YAHPEAUlua_State@@HPEBD@Z"]
    pub fn luaL_callmeta(L: *mut lua_State, obj: c_int, e: *const c_char) -> c_int;
    #[link_name = "?luaL_typerror@@YAHPEAUlua_State@@HPEBD@Z"]
    pub fn luaL_typerror(L: *mut lua_State, narg: c_int, tname: *const c_char) -> c_int;
    #[link_name = "?luaL_argerror@@YAHPEAUlua_State@@HPEBD@Z"]
    pub fn luaL_argerror(L: *mut lua_State, narg: c_int, extramsg: *const c_char) -> c_int;
    #[link_name = "?luaL_checklstring@@YAPEBDPEAUlua_State@@HPEA_K@Z"]
    pub fn luaL_checklstring(L: *mut lua_State, narg: c_int, l: *mut usize) -> *const c_char;
    #[link_name = "?luaL_optlstring@@YAPEBDPEAUlua_State@@HPEBDPEA_K@Z"]
    pub fn luaL_optlstring(
        L: *mut lua_State,
        narg: c_int,
        def: *const c_char,
        l: *mut usize,
    ) -> *const c_char;
    #[link_name = "?luaL_checknumber@@YANPEAUlua_State@@H@Z"]
    pub fn luaL_checknumber(L: *mut lua_State, narg: c_int) -> lua_Number;
    #[link_name = "?luaL_optnumber@@YANPEAUlua_State@@HN@Z"]
    pub fn luaL_optnumber(L: *mut lua_State, narg: c_int, def: lua_Number) -> lua_Number;
    #[link_name = "?luaL_checkinteger@@YAHPEAUlua_State@@H@Z"]
    pub fn luaL_checkinteger(L: *mut lua_State, narg: c_int) -> lua_Integer;
    #[link_name = "?luaL_optinteger@@YAHPEAUlua_State@@HH@Z"]
    pub fn luaL_optinteger(L: *mut lua_State, narg: c_int, def: lua_Integer) -> lua_Integer;
    #[link_name = "?luaL_checkstack@@YAXPEAUlua_State@@HPEBD@Z"]
    pub fn luaL_checkstack(L: *mut lua_State, sz: c_int, msg: *const c_char);
    #[link_name = "?luaL_checktype@@YAXPEAUlua_State@@HH@Z"]
    pub fn luaL_checktype(L: *mut lua_State, narg: c_int, t: c_int);
    #[link_name = "?luaL_checkany@@YAXPEAUlua_State@@H@Z"]
    pub fn luaL_checkany(L: *mut lua_State, narg: c_int);

    #[link_name = "?luaL_newmetatable@@YAHPEAUlua_State@@PEBD@Z"]
    pub fn luaL_newmetatable_(L: *mut lua_State, tname: *const c_char) -> c_int;
    #[link_name = "?luaL_checkudata@@YAPEAXPEAUlua_State@@HPEBD@Z"]
    pub fn luaL_checkudata(L: *mut lua_State, ud: c_int, tname: *const c_char) -> *mut c_void;

    #[link_name = "?luaL_where@@YAXPEAUlua_State@@H@Z"]
    pub fn luaL_where(L: *mut lua_State, lvl: c_int);
    #[link_name = "?luaL_error@@YAHPEAUlua_State@@PEBDZZ"]
    pub fn luaL_error(L: *mut lua_State, fmt: *const c_char, ...) -> c_int;

    #[link_name = "?luaL_checkoption@@YAHPEAUlua_State@@HPEBDQEBQEBD@Z"]
    pub fn luaL_checkoption(
        L: *mut lua_State,
        narg: c_int,
        def: *const c_char,
        lst: *const *const c_char,
    ) -> c_int;
}

// Pre-defined references
pub const LUA_NOREF: c_int = -2;
pub const LUA_REFNIL: c_int = -1;

#[cfg_attr(
    all(windows, raw_dylib),
    link(name = "HavokScript_FinalRelease", kind = "raw-dylib")
)]
extern "C-unwind" {
    #[link_name = "?luaL_ref@@YAHPEAUlua_State@@H@Z"]
    pub fn luaL_ref(L: *mut lua_State, t: c_int) -> c_int;
    #[link_name = "?luaL_unref@@YAXPEAUlua_State@@HH@Z"]
    pub fn luaL_unref(L: *mut lua_State, t: c_int, r#ref: c_int);

    #[link_name = "?luaL_loadfile@@YAHPEAUlua_State@@PEBD@Z"]
    pub fn luaL_loadfile(L: *mut lua_State, filename: *const c_char) -> c_int;
    #[link_name = "?luaL_loadbuffer@@YAHPEAUlua_State@@PEBD_K1@Z"]
    pub fn luaL_loadbuffer(
        L: *mut lua_State,
        buff: *const c_char,
        sz: usize,
        name: *const c_char,
    ) -> c_int;
    #[link_name = "?luaL_loadstring@@YAHPEAUlua_State@@PEBD@Z"]
    pub fn luaL_loadstring(L: *mut lua_State, s: *const c_char) -> c_int;

    #[link_name = "?luaL_newstate@@YAPEAUlua_State@@XZ"]
    pub fn luaL_newstate() -> *mut lua_State;

    #[link_name = "?luaL_gsub@@YAPEBDPEAUlua_State@@PEBD11@Z"]
    pub fn luaL_gsub(
        L: *mut lua_State,
        s: *const c_char,
        p: *const c_char,
        r: *const c_char,
    ) -> *const c_char;

    #[link_name = "?luaL_findtable@@YAPEBDPEAUlua_State@@HPEBDH@Z"]
    pub fn luaL_findtable(
        L: *mut lua_State,
        idx: c_int,
        fname: *const c_char,
        szhint: c_int,
    ) -> *const c_char;
}

//
// Some useful macros (implemented as Rust functions)
//

#[inline(always)]
pub unsafe fn luaL_argcheck(L: *mut lua_State, cond: c_int, narg: c_int, extramsg: *const c_char) {
    if cond == 0 {
        luaL_argerror(L, narg, extramsg);
    }
}

#[inline(always)]
pub unsafe fn luaL_checkstring(L: *mut lua_State, n: c_int) -> *const c_char {
    luaL_checklstring(L, n, ptr::null_mut())
}

#[inline(always)]
pub unsafe fn luaL_optstring(L: *mut lua_State, n: c_int, d: *const c_char) -> *const c_char {
    luaL_optlstring(L, n, d, ptr::null_mut())
}

// Deprecated from 5.3: luaL_checkint, luaL_optint, luaL_checklong, luaL_optlong

#[inline(always)]
pub unsafe fn luaL_typename(L: *mut lua_State, i: c_int) -> *const c_char {
    lua::lua_typename(L, lua::lua_type(L, i))
}

pub unsafe fn luaL_dofile(L: *mut lua_State, filename: *const c_char) -> c_int {
    let status = luaL_loadfile(L, filename);
    if status == 0 {
        lua::lua_pcall(L, 0, lua::LUA_MULTRET, 0)
    } else {
        status
    }
}

#[inline(always)]
pub unsafe fn luaL_dostring(L: *mut lua_State, s: *const c_char) -> c_int {
    let status = luaL_loadstring(L, s);
    if status == 0 {
        lua::lua_pcall(L, 0, lua::LUA_MULTRET, 0)
    } else {
        status
    }
}

#[inline(always)]
pub unsafe fn luaL_getmetatable(L: *mut lua_State, n: *const c_char) {
    lua::lua_getfield_(L, lua::LUA_REGISTRYINDEX, n);
}

// TODO: luaL_opt

//
// TODO: Generic Buffer Manipulation
//

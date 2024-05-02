//! Contains definitions from `lua.h`.

use std::marker::{PhantomData, PhantomPinned};
use std::mem::MaybeUninit;
use std::os::raw::{c_char, c_double, c_int, c_void};
use std::ptr;

// Mark for precompiled code (`<esc>Lua`)
pub const LUA_SIGNATURE: &[u8] = b"\x1bLua";

// Option for multiple returns in 'lua_pcall' and 'lua_call'
pub const LUA_MULTRET: c_int = -1;

//
// Pseudo-indices
//
pub const LUA_REGISTRYINDEX: c_int = -10000;
pub const LUA_ENVIRONINDEX: c_int = -10001;
pub const LUA_GLOBALSINDEX: c_int = -10002;

pub const fn lua_upvalueindex(i: c_int) -> c_int {
    LUA_GLOBALSINDEX - i
}

//
// Thread status
//
pub const LUA_OK: c_int = 0;
pub const LUA_YIELD: c_int = 1;
pub const LUA_ERRRUN: c_int = 2;
pub const LUA_ERRSYNTAX: c_int = 3;
pub const LUA_ERRMEM: c_int = 4;
pub const LUA_ERRERR: c_int = 5;

/// A raw Lua state associated with a thread.
#[repr(C)]
pub struct lua_State {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

//
// Basic types
//
pub const LUA_TNONE: c_int = -1;

pub const LUA_TNIL: c_int = 0;
pub const LUA_TBOOLEAN: c_int = 1;
pub const LUA_TLIGHTUSERDATA: c_int = 2;
pub const LUA_TNUMBER: c_int = 3;
pub const LUA_TSTRING: c_int = 4;
pub const LUA_TTABLE: c_int = 5;
pub const LUA_TFUNCTION: c_int = 6;
pub const LUA_TUSERDATA: c_int = 7;
pub const LUA_TTHREAD: c_int = 8;

/// Minimum Lua stack available to a C function
pub const LUA_MINSTACK: c_int = 20;

/// A Lua number, usually equivalent to `f64`
pub type lua_Number = c_double;

/// A Lua integer, usually equivalent to `i64`
#[cfg(target_pointer_width = "32")]
pub type lua_Integer = i32;
#[cfg(target_pointer_width = "64")]
pub type lua_Integer = i64;

/// Type for native C functions that can be passed to Lua.
pub type lua_CFunction = unsafe extern "C-unwind" fn(L: *mut lua_State) -> c_int;

// Type for functions that read/write blocks when loading/dumping Lua chunks
#[rustfmt::skip]
pub type lua_Reader =
    unsafe extern "C-unwind" fn(L: *mut lua_State, ud: *mut c_void, sz: *mut usize) -> *const c_char;
#[rustfmt::skip]
pub type lua_Writer =
    unsafe extern "C-unwind" fn(L: *mut lua_State, p: *const c_void, sz: usize, ud: *mut c_void) -> c_int;

/// Type for memory-allocation functions
#[rustfmt::skip]
pub type lua_Alloc =
    unsafe extern "C-unwind" fn(ud: *mut c_void, ptr: *mut c_void, osize: usize, nsize: usize) -> *mut c_void;

#[cfg_attr(
    all(windows, raw_dylib),
    link(name = "HavokScript_FinalRelease", kind = "raw-dylib")
)]
extern "C-unwind" {
    //
    // State manipulation
    //
    #[link_name = "?lua_newstate@@YAPEAUlua_State@@P6APEAXPEAX0_K1@Z0@Z"]
    pub fn lua_newstate(f: lua_Alloc, ud: *mut c_void) -> *mut lua_State;
    #[link_name = "?lua_close@@YAXPEAUlua_State@@@Z"]
    pub fn lua_close(L: *mut lua_State);
    #[link_name = "?lua_newthread@@YAPEAUlua_State@@PEAU1@@Z"]
    pub fn lua_newthread(L: *mut lua_State) -> *mut lua_State;

    #[link_name = "?lua_atpanic@@YAP6AHPEAUlua_State@@@Z0P6AH0@Z@Z"]
    pub fn lua_atpanic(L: *mut lua_State, panicf: lua_CFunction) -> lua_CFunction;

    //
    // Basic stack manipulation
    //
    #[link_name = "?lua_gettop@@YAHPEAUlua_State@@@Z"]
    pub fn lua_gettop(L: *mut lua_State) -> c_int;
    #[link_name = "?lua_settop@@YAXPEAUlua_State@@H@Z"]
    pub fn lua_settop(L: *mut lua_State, idx: c_int);
    #[link_name = "?lua_pushvalue@@YAXPEAUlua_State@@H@Z"]
    pub fn lua_pushvalue(L: *mut lua_State, idx: c_int);
    #[link_name = "?lua_remove@@YAXPEAUlua_State@@H@Z"]
    pub fn lua_remove(L: *mut lua_State, idx: c_int);
    #[link_name = "?lua_insert@@YAXPEAUlua_State@@H@Z"]
    pub fn lua_insert(L: *mut lua_State, idx: c_int);
    #[link_name = "?lua_replace@@YAXPEAUlua_State@@H@Z"]
    pub fn lua_replace(L: *mut lua_State, idx: c_int);

    // pub fn lua_checkstack(L: *mut lua_State, sz: c_int) -> c_int;

    // pub fn lua_xmove(from: *mut lua_State, to: *mut lua_State, n: c_int);

    //
    // Access functions (stack -> C)
    //
    #[link_name = "?lua_isnumber@@YAHPEAUlua_State@@H@Z"]
    pub fn lua_isnumber(L: *mut lua_State, idx: c_int) -> c_int;
    #[link_name = "?lua_isstring@@YAHPEAUlua_State@@H@Z"]
    pub fn lua_isstring(L: *mut lua_State, idx: c_int) -> c_int;
    #[link_name = "?lua_iscfunction@@YAHPEAUlua_State@@H@Z"]
    pub fn lua_iscfunction(L: *mut lua_State, idx: c_int) -> c_int;
    #[link_name = "?lua_isuserdata@@YAHPEAUlua_State@@H@Z"]
    pub fn lua_isuserdata(L: *mut lua_State, idx: c_int) -> c_int;
    #[link_name = "?lua_type@@YAHPEAUlua_State@@H@Z"]
    pub fn lua_type(L: *mut lua_State, idx: c_int) -> c_int;
    #[link_name = "?lua_typename@@YAPEBDPEAUlua_State@@H@Z"]
    pub fn lua_typename(L: *mut lua_State, tp: c_int) -> *const c_char;

    #[link_name = "?lua_equal@@YAHPEAUlua_State@@HH@Z"]
    pub fn lua_equal(L: *mut lua_State, idx1: c_int, idx2: c_int) -> c_int;
    #[link_name = "?lua_rawequal@@YAHPEAUlua_State@@HH@Z"]
    pub fn lua_rawequal(L: *mut lua_State, idx1: c_int, idx2: c_int) -> c_int;
    #[link_name = "?lua_lessthan@@YAHPEAUlua_State@@HH@Z"]
    pub fn lua_lessthan(L: *mut lua_State, idx1: c_int, idx2: c_int) -> c_int;

    #[link_name = "?lua_tonumber@@YANPEAUlua_State@@H@Z"]
    pub fn lua_tonumber(L: *mut lua_State, idx: c_int) -> lua_Number;
    #[link_name = "?lua_tointeger@@YAHPEAUlua_State@@H@Z"]
    pub fn lua_tointeger_(L: *mut lua_State, idx: c_int) -> lua_Integer;
    #[link_name = "?lua_toboolean@@YA_NPEAUlua_State@@H@Z"]
    pub fn lua_toboolean(L: *mut lua_State, idx: c_int) -> c_int;
    #[link_name = "?lua_tolstring@@YAPEBDPEAUlua_State@@HPEA_K@Z"]
    pub fn lua_tolstring(L: *mut lua_State, idx: c_int, len: *mut usize) -> *const c_char;
    #[link_name = "?lua_objlen@@YA_KPEAUlua_State@@H@Z"]
    pub fn lua_objlen(L: *mut lua_State, idx: c_int) -> usize;
    #[link_name = "?lua_tocfunction@@YAP6AHPEAUlua_State@@@Z0H@Z"]
    pub fn lua_tocfunction(L: *mut lua_State, idx: c_int) -> Option<lua_CFunction>;
    #[link_name = "?lua_touserdata@@YAPEAXPEAUlua_State@@H@Z"]
    pub fn lua_touserdata(L: *mut lua_State, idx: c_int) -> *mut c_void;
    #[link_name = "?lua_tothread@@YAPEAUlua_State@@PEAU1@H@Z"]
    pub fn lua_tothread(L: *mut lua_State, idx: c_int) -> *mut lua_State;
    #[link_name = "?lua_topointer@@YAPEBXPEAUlua_State@@H@Z"]
    pub fn lua_topointer(L: *mut lua_State, idx: c_int) -> *const c_void;

    //
    // Push functions (C -> stack)
    //
    #[link_name = "?lua_pushnil@@YAXPEAUlua_State@@@Z"]
    pub fn lua_pushnil(L: *mut lua_State);
    #[link_name = "?lua_pushnumber@@YAXPEAUlua_State@@N@Z"]
    pub fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
    #[link_name = "?lua_pushinteger@@YAXPEAUlua_State@@H@Z"]
    pub fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);

    // #[link_name = "lua_pushlstring"]
    // pub fn lua_pushlstring_(L: *mut lua_State, s: *const c_char, l: usize);
    // #[link_name = "lua_pushstring"]
    // pub fn lua_pushstring_(L: *mut lua_State, s: *const c_char);
    // lua_pushvfstring
    #[link_name = "?lua_pushfstring@@YAPEBDPEAUlua_State@@PEBDZZ"]
    pub fn lua_pushfstring(L: *mut lua_State, fmt: *const c_char, ...) -> *const c_char;
    // pub fn lua_pushcclosure(L: *mut lua_State, f: lua_CFunction, n: c_int);

    // pub fn lua_pushboolean(L: *mut lua_State, b: c_int);
    #[link_name = "?lua_pushlightuserdata@@YAXPEAUlua_State@@PEAX@Z"]
    pub fn lua_pushlightuserdata(L: *mut lua_State, p: *mut c_void);
    #[link_name = "?lua_pushthread@@YAHPEAUlua_State@@@Z"]
    pub fn lua_pushthread(L: *mut lua_State) -> c_int;

    //
    // Get functions (Lua -> stack)
    //
    #[link_name = "?lua_gettable@@YAXPEAUlua_State@@H@Z"]
    pub fn lua_gettable_(L: *mut lua_State, idx: c_int);
    #[link_name = "?lua_getfield@@YAXPEAUlua_State@@HPEBD@Z"]
    pub fn lua_getfield_(L: *mut lua_State, idx: c_int, k: *const c_char);
    #[link_name = "?lua_rawget@@YAXPEAUlua_State@@H@Z"]
    pub fn lua_rawget_(L: *mut lua_State, idx: c_int);
    #[link_name = "?lua_rawgeti@@YAXPEAUlua_State@@HH@Z"]
    pub fn lua_rawgeti_(L: *mut lua_State, idx: c_int, n: c_int);
    #[link_name = "?lua_createtable@@YAXPEAUlua_State@@HH@Z"]
    pub fn lua_createtable(L: *mut lua_State, narr: c_int, nrec: c_int);
    #[link_name = "?lua_newuserdata@@YAPEAXPEAUlua_State@@_K@Z"]
    pub fn lua_newuserdata(L: *mut lua_State, sz: usize) -> *mut c_void;
    #[link_name = "?lua_getmetatable@@YAHPEAUlua_State@@H@Z"]
    pub fn lua_getmetatable(L: *mut lua_State, objindex: c_int) -> c_int;
    #[link_name = "?lua_getfenv@@YAXPEAUlua_State@@H@Z"]
    pub fn lua_getfenv(L: *mut lua_State, idx: c_int);

    //
    // Set functions (stack -> Lua)
    //
    // pub fn lua_settable(L: *mut lua_State, idx: c_int);
    #[link_name = "?lua_setfield@@YAXPEAUlua_State@@HPEBD@Z"]
    pub fn lua_setfield(L: *mut lua_State, idx: c_int, k: *const c_char);
    #[link_name = "?lua_rawset@@YAXPEAUlua_State@@H@Z"]
    pub fn lua_rawset(L: *mut lua_State, idx: c_int);
    #[link_name = "?lua_rawseti@@YAXPEAUlua_State@@HH@Z"]
    pub fn lua_rawseti_(L: *mut lua_State, idx: c_int, n: c_int);
    #[link_name = "?lua_setmetatable@@YAHPEAUlua_State@@H@Z"]
    pub fn lua_setmetatable(L: *mut lua_State, objindex: c_int) -> c_int;
    #[link_name = "?lua_setfenv@@YAHPEAUlua_State@@H@Z"]
    pub fn lua_setfenv(L: *mut lua_State, idx: c_int) -> c_int;

    //
    // 'load' and 'call' functions (load and run Lua code)
    //
    #[link_name = "?lua_call@@YAXPEAUlua_State@@HH@Z"]
    pub fn lua_call(L: *mut lua_State, nargs: c_int, nresults: c_int);
    #[link_name = "?lua_pcall@@YAHPEAUlua_State@@HHH@Z"]
    pub fn lua_pcall(L: *mut lua_State, nargs: c_int, nresults: c_int, errfunc: c_int) -> c_int;
    #[link_name = "?lua_cpcall@@YAHPEAUlua_State@@P6AH0@ZPEAX@Z"]
    pub fn lua_cpcall(L: *mut lua_State, f: lua_CFunction, ud: *mut c_void) -> c_int;
    #[link_name = "?lua_load@@YAHPEAUlua_State@@P6APEBD0PEAXPEA_K@Z1PEBD@Z"]
    pub fn lua_load(
        L: *mut lua_State,
        reader: lua_Reader,
        data: *mut c_void,
        chunkname: *const c_char,
    ) -> c_int;

    #[link_name = "?lua_dump@@YAHPEAUlua_State@@P6AH0PEBX_KPEAX@Z3@Z"]
    pub fn lua_dump_(L: *mut lua_State, writer: lua_Writer, data: *mut c_void) -> c_int;

    //
    // Coroutine functions
    //
    #[link_name = "?lua_yield@@YAHPEAUlua_State@@H@Z"]
    pub fn lua_yield(L: *mut lua_State, nresults: c_int) -> c_int;
    #[link_name = "?lua_resume@@YAHPEAUlua_State@@H@Z"]
    pub fn lua_resume_(L: *mut lua_State, narg: c_int) -> c_int;
    // pub fn lua_status(L: *mut lua_State) -> c_int;

    // Missing function workarounds
    #[link_name = "?CheckStack@LuaState@LuaPlus@@QEAAHH@Z"]
    pub fn CheckStack(L: *mut lua_State, sz: c_int) -> c_int;
    #[link_name = "?XMove@LuaState@LuaPlus@@QEAAXPEAV12@H@Z"]
    pub fn XMove(L: *mut lua_State, to: *mut lua_State, n: c_int);
    #[link_name = "?SetTable@LuaState@LuaPlus@@QEAAXH@Z"]
    pub fn SetTable(L: *mut lua_State, idx: c_int);
    #[link_name = "?CoStatus@LuaState@LuaPlus@@QEAAHXZ"]
    pub fn CoStatus(L: *mut lua_State) -> c_int;

    // The following functions use a different calling convention for the return value
    // which must be manually accounted for by passing a pointer to the return value
    #[link_name = "?PushLString@LuaState@LuaPlus@@QEAA?AVLuaStackObject@2@PEBD_K@Z"]
    pub fn PushLString(
        L: *mut lua_State,
        ret: *mut LuaStackObject,
        s: *const c_char,
        l: usize,
    ) -> *mut LuaStackObject;
    #[link_name = "?PushString@LuaState@LuaPlus@@QEAA?AVLuaStackObject@2@PEBD@Z"]
    pub fn PushString(
        L: *mut lua_State,
        ret: *mut LuaStackObject,
        s: *const c_char,
    ) -> *mut LuaStackObject;
    #[link_name = "?PushCClosure@LuaState@LuaPlus@@QEAA?AVLuaStackObject@2@P6AHPEAUlua_State@@@ZH@Z"]
    pub fn PushCClosure(
        L: *mut lua_State,
        ret: *mut LuaStackObject,
        f: lua_CFunction,
        n: c_int,
    ) -> *mut LuaStackObject;
    #[link_name = "?PushBoolean@LuaState@LuaPlus@@QEAA?AVLuaStackObject@2@_N@Z"]
    pub fn PushBoolean(
        L: *mut lua_State,
        ret: *mut LuaStackObject,
        b: c_char,
    ) -> *mut LuaStackObject;
}

// Structs for workaround function signatures
#[repr(C)]
pub struct LuaStackObject {
    L: *mut lua_State,
    m_stackIndex: c_int,
}

//
// Garbage-collection function and options
//
pub const LUA_GCSTOP: c_int = 0;
pub const LUA_GCRESTART: c_int = 1;
pub const LUA_GCCOLLECT: c_int = 2;
pub const LUA_GCCOUNT: c_int = 3;
pub const LUA_GCCOUNTB: c_int = 4;
pub const LUA_GCSTEP: c_int = 5;
pub const LUA_GCSETPAUSE: c_int = 6;
pub const LUA_GCSETSTEPMUL: c_int = 7;

#[cfg_attr(
    all(windows, raw_dylib),
    link(name = "HavokScript_FinalRelease", kind = "raw-dylib")
)]
extern "C-unwind" {
    #[link_name = "?lua_gc@@YAHPEAUlua_State@@HH@Z"]
    pub fn lua_gc(L: *mut lua_State, what: c_int, data: c_int) -> c_int;
}

//
// Miscellaneous functions
//
#[cfg_attr(
    all(windows, raw_dylib),
    link(name = "HavokScript_FinalRelease", kind = "raw-dylib")
)]
extern "C-unwind" {
    #[link_name = "?lua_error@@YAHPEAUlua_State@@@Z"]
    fn lua_error_(L: *mut lua_State) -> c_int;
    #[link_name = "?lua_next@@YAHPEAUlua_State@@H@Z"]
    pub fn lua_next(L: *mut lua_State, idx: c_int) -> c_int;
    #[link_name = "?lua_concat@@YAXPEAUlua_State@@H@Z"]
    pub fn lua_concat(L: *mut lua_State, n: c_int);
    #[link_name = "?lua_getallocf@@YAP6APEAXPEAX0_K1@ZPEAUlua_State@@PEAPEAX@Z"]
    pub fn lua_getallocf(L: *mut lua_State, ud: *mut *mut c_void) -> lua_Alloc;
    #[link_name = "?lua_setallocf@@YAXPEAUlua_State@@P6APEAXPEAX1_K2@Z1@Z"]
    pub fn lua_setallocf(L: *mut lua_State, f: lua_Alloc, ud: *mut c_void);
}

// lua_error does not return but is declared to return int, and Rust translates
// ! to void which can cause link-time errors if the platform linker is aware
// of return types and requires they match (for example: wasm does this).
#[inline(always)]
pub unsafe fn lua_error(L: *mut lua_State) -> ! {
    lua_error_(L);
    unreachable!();
}

//
// Workarounds for missing functions
//
#[inline(always)]
pub unsafe fn lua_checkstack(L: *mut lua_State, sz: c_int) -> c_int {
    CheckStack(L, sz)
}

#[inline(always)]
pub unsafe fn lua_xmove(from: *mut lua_State, to: *mut lua_State, n: c_int) {
    XMove(from, to, n);
}

#[inline(always)]
pub unsafe fn lua_pushlstring_(L: *mut lua_State, s: *const c_char, l: usize) {
    let mut ret = MaybeUninit::uninit();

    PushLString(L, ret.as_mut_ptr(), s, l);

    ret.assume_init();
}

#[inline(always)]
pub unsafe fn lua_pushstring_(L: *mut lua_State, s: *const c_char) {
    let mut ret = MaybeUninit::uninit();

    PushString(L, ret.as_mut_ptr(), s);

    ret.assume_init();
}

#[inline(always)]
pub unsafe fn lua_pushcclosure(L: *mut lua_State, f: lua_CFunction, n: c_int) {
    let mut ret = MaybeUninit::uninit();

    PushCClosure(L, ret.as_mut_ptr(), f, n);

    ret.assume_init();
}

#[inline(always)]
pub unsafe fn lua_pushboolean(L: *mut lua_State, b: c_int) {
    let mut ret = MaybeUninit::uninit();

    PushBoolean(L, ret.as_mut_ptr(), b as c_char);

    ret.assume_init();
}

#[inline(always)]
pub unsafe fn lua_settable(L: *mut lua_State, idx: c_int) {
    SetTable(L, idx);
}

#[inline(always)]
pub unsafe fn lua_status(L: *mut lua_State) -> c_int {
    CoStatus(L)
}

//
// Some useful macros (implemented as Rust functions)
//
#[inline(always)]
pub unsafe fn lua_pop(L: *mut lua_State, n: c_int) {
    lua_settop(L, -n - 1)
}

#[inline(always)]
pub unsafe fn lua_newtable(L: *mut lua_State) {
    lua_createtable(L, 0, 0)
}

#[inline(always)]
pub unsafe fn lua_register(L: *mut lua_State, n: *const c_char, f: lua_CFunction) {
    lua_pushcfunction(L, f);
    lua_setglobal(L, n)
}

#[inline(always)]
pub unsafe fn lua_pushcfunction(L: *mut lua_State, f: lua_CFunction) {
    lua_pushcclosure(L, f, 0)
}

// TODO: lua_strlen

#[inline(always)]
pub unsafe fn lua_isfunction(L: *mut lua_State, n: c_int) -> c_int {
    (lua_type(L, n) == LUA_TFUNCTION) as c_int
}

#[inline(always)]
pub unsafe fn lua_istable(L: *mut lua_State, n: c_int) -> c_int {
    (lua_type(L, n) == LUA_TTABLE) as c_int
}

#[inline(always)]
pub unsafe fn lua_islightuserdata(L: *mut lua_State, n: c_int) -> c_int {
    (lua_type(L, n) == LUA_TLIGHTUSERDATA) as c_int
}

#[inline(always)]
pub unsafe fn lua_isnil(L: *mut lua_State, n: c_int) -> c_int {
    (lua_type(L, n) == LUA_TNIL) as c_int
}

#[inline(always)]
pub unsafe fn lua_isboolean(L: *mut lua_State, n: c_int) -> c_int {
    (lua_type(L, n) == LUA_TBOOLEAN) as c_int
}

#[inline(always)]
pub unsafe fn lua_isthread(L: *mut lua_State, n: c_int) -> c_int {
    (lua_type(L, n) == LUA_TTHREAD) as c_int
}

#[inline(always)]
pub unsafe fn lua_isnone(L: *mut lua_State, n: c_int) -> c_int {
    (lua_type(L, n) == LUA_TNONE) as c_int
}

#[inline(always)]
pub unsafe fn lua_isnoneornil(L: *mut lua_State, n: c_int) -> c_int {
    (lua_type(L, n) <= LUA_TNIL) as c_int
}

#[inline(always)]
pub unsafe fn lua_pushliteral(L: *mut lua_State, s: &'static str) {
    use std::ffi::CString;
    let c_str = CString::new(s).unwrap();
    lua_pushlstring_(L, c_str.as_ptr(), c_str.as_bytes().len())
}

#[inline(always)]
pub unsafe fn lua_setglobal(L: *mut lua_State, var: *const c_char) {
    lua_setfield(L, LUA_GLOBALSINDEX, var)
}

#[inline(always)]
pub unsafe fn lua_getglobal_(L: *mut lua_State, var: *const c_char) {
    lua_getfield_(L, LUA_GLOBALSINDEX, var)
}

#[inline(always)]
pub unsafe fn lua_tolightuserdata(L: *mut lua_State, idx: c_int) -> *mut c_void {
    if lua_islightuserdata(L, idx) != 0 {
        return lua_touserdata(L, idx);
    }
    ptr::null_mut()
}

#[inline(always)]
pub unsafe fn lua_tostring(L: *mut lua_State, i: c_int) -> *const c_char {
    lua_tolstring(L, i, ptr::null_mut())
}

#[inline(always)]
pub unsafe fn lua_xpush(from: *mut lua_State, to: *mut lua_State, idx: c_int) {
    lua_pushvalue(from, idx);
    lua_xmove(from, to, 1);
}

//
// Debug API
//

// Maximum size for the description of the source of a function in debug information.
const LUA_IDSIZE: usize = 60;

// Event codes
pub const LUA_HOOKCALL: c_int = 0;
pub const LUA_HOOKRET: c_int = 1;
pub const LUA_HOOKLINE: c_int = 2;
pub const LUA_HOOKCOUNT: c_int = 3;
pub const LUA_HOOKTAILCALL: c_int = 4;

// Event masks
pub const LUA_MASKCALL: c_int = 1 << (LUA_HOOKCALL as usize);
pub const LUA_MASKRET: c_int = 1 << (LUA_HOOKRET as usize);
pub const LUA_MASKLINE: c_int = 1 << (LUA_HOOKLINE as usize);
pub const LUA_MASKCOUNT: c_int = 1 << (LUA_HOOKCOUNT as usize);

/// Type for functions to be called on debug events.
pub type lua_Hook = unsafe extern "C-unwind" fn(L: *mut lua_State, ar: *mut lua_Debug);

#[cfg_attr(
    all(windows, raw_dylib),
    link(name = "HavokScript_FinalRelease", kind = "raw-dylib")
)]
extern "C-unwind" {
    #[link_name = "?lua_getstack@@YAHPEAUlua_State@@HPEAUlua_Debug@@@Z"]
    pub fn lua_getstack(L: *mut lua_State, level: c_int, ar: *mut lua_Debug) -> c_int;
    #[link_name = "?lua_getinfo@@YAHPEAUlua_State@@PEBDPEAUlua_Debug@@@Z"]
    pub fn lua_getinfo(L: *mut lua_State, what: *const c_char, ar: *mut lua_Debug) -> c_int;
    #[link_name = "?lua_getlocal@@YAPEBDPEAUlua_State@@PEAUlua_Debug@@H@Z"]
    pub fn lua_getlocal(L: *mut lua_State, ar: *const lua_Debug, n: c_int) -> *const c_char;
    #[link_name = "?lua_setlocal@@YAPEBDPEAUlua_State@@PEAUlua_Debug@@H@Z"]
    pub fn lua_setlocal(L: *mut lua_State, ar: *const lua_Debug, n: c_int) -> *const c_char;
    #[link_name = "?lua_getupvalue@@YAPEBDPEAUlua_State@@HH@Z"]
    pub fn lua_getupvalue(L: *mut lua_State, funcindex: c_int, n: c_int) -> *const c_char;
    #[link_name = "?lua_setupvalue@@YAPEBDPEAUlua_State@@HH@Z"]
    pub fn lua_setupvalue(L: *mut lua_State, funcindex: c_int, n: c_int) -> *const c_char;

    #[link_name = "?lua_sethook@@YAHPEAUlua_State@@P6AX0PEAUlua_Debug@@@ZHH@Z"]
    pub fn lua_sethook(
        L: *mut lua_State,
        func: Option<lua_Hook>,
        mask: c_int,
        count: c_int,
    ) -> c_int;
    #[link_name = "?lua_gethook@@YAP6AXPEAUlua_State@@PEAUlua_Debug@@@Z0@Z"]
    pub fn lua_gethook(L: *mut lua_State) -> Option<lua_Hook>;
    #[link_name = "?lua_gethookmask@@YAHPEAUlua_State@@@Z"]
    pub fn lua_gethookmask(L: *mut lua_State) -> c_int;
    #[link_name = "?lua_gethookcount@@YAHPEAUlua_State@@@Z"]
    pub fn lua_gethookcount(L: *mut lua_State) -> c_int;
}

#[repr(C)]
pub struct lua_Debug {
    pub event: c_int,
    pub name: *const c_char,
    pub namewhat: *const c_char,
    pub what: *const c_char,
    pub source: *const c_char,
    pub currentline: c_int,
    pub nups: c_int,
    pub linedefined: c_int,
    pub lastlinedefined: c_int,
    pub short_src: [c_char; LUA_IDSIZE],
    // lua.h mentions this is for private use
    i_ci: c_int,
}

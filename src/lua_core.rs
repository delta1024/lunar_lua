use core::str;
use std::{
    ffi::{CStr, CString},
    marker::PhantomData,
    mem,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use crate::{
    ffi::{
        lua_Alloc, lua_CFunction, lua_State, lua_checkstack, lua_copy, lua_createtable, lua_error,
        lua_getglobal, lua_gettable, lua_gettop, lua_iscfunction, lua_newstate, lua_newuserdatauv,
        lua_pcallk, lua_pushboolean, lua_pushcclosure, lua_pushlstring, lua_pushnil,
        lua_pushnumber, lua_pushstring, lua_pushvalue, lua_rotate, lua_setglobal, lua_settable,
        lua_settop, lua_toboolean, lua_tolstring, lua_tonumberx, lua_type, lua_typename, LUA_OK,
    },
    LuaConn, LuaError, LuaStateRef, LuaType,
};

#[macro_export]
macro_rules! check_for_err {
    ($result:tt) => {
        if $result != LUA_OK as i32 {
            return Err(unsafe { std::mem::transmute($result) });
        }
    };
}
/// Creates a new independent state and returns its main thread. Returns NULL if it cannot create the state (due to lack of memory). The argument f is the allocator function; Lua will do all memory allocation for this state through this function (see lua_Alloc). The second argument, ud, is an opaque pointer that Lua passes to the allocator in every call.
pub fn new_state(f: lua_Alloc, ud: *mut ::std::os::raw::c_void) -> *mut lua_State {
    unsafe { lua_newstate(f, ud) }
}
pub struct UserData<'state, T: ?Sized>(NonNull<T>, PhantomData<LuaStateRef<'state>>);
impl<'state, T: ?Sized> UserData<'state, T> {
    pub fn new(ptr: *mut T) -> Self {
        unsafe { Self(NonNull::new_unchecked(ptr), PhantomData) }
    }
}
impl<'state, T: ?Sized> Deref for UserData<'state, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}
impl<'state, T: ?Sized> DerefMut for UserData<'state, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}
///  a container for valid stack types
pub enum LuaStackValue<'a> {
    Number(f64),
    Bool(bool),
    String(&'a str),
    CFunction(lua_CFunction),
    Nil,
}
impl From<f64> for LuaStackValue<'_> {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
impl From<bool> for LuaStackValue<'_> {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}
impl<'a> From<&'a str> for LuaStackValue<'a> {
    fn from(value: &'a str) -> Self {
        Self::String(value)
    }
}
impl From<()> for LuaStackValue<'_> {
    fn from(_: ()) -> Self {
        Self::Nil
    }
}
impl From<lua_CFunction> for LuaStackValue<'_> {
    fn from(value: lua_CFunction) -> Self {
        Self::CFunction(value)
    }
}
pub trait LuaCore: LuaConn {
    /// Creates a new empty table and pushes it onto the stack. It is equivalent to [lua_createtable(L, 0, 0)].
    fn new_table(&self) {
        unsafe {
            lua_createtable(self.get_conn().get_mut_ptr(), 0, 0);
        }
    }
    ///  This function creates and pushes on the stack a new full userdata, with nuvalue associated Lua values, called user values, plus an associated block of raw memory with size bytes. (The user values can be set and read with the functions lua_setiuservalue and lua_getiuservalue.)
    ///
    /// The function returns the address of the block of memory. Lua ensures that this address is valid as long as the corresponding userdata is alive (see §2.5). Moreover, if the userdata is marked for finalization (see §2.5.3), its address is valid at least until the call to its finalizer.
    fn new_user_data<'state, T>(&'state self, data: T) -> UserData<'state, T> {
        unsafe {
            let size = mem::size_of::<T>();
            let ptr: *mut T = lua_newuserdatauv(self.get_conn().get_mut_ptr(), size, 1).cast();
            ptr.write(data);
            UserData(NonNull::new_unchecked(ptr), PhantomData)
        }
    }
    ///  Calls a function (or a callable object) in protected mode.
    ///
    /// Both nargs and nresults have the same meaning as in [lua_call](crate::ffi::lua_callk). If there are no errors during the call, lua_pcall behaves exactly like [lua_call](crate::ffi::lua_callk). However, if there is any error, [pcall](LuaCore::pcall) catches it, pushes a single value on the stack (the error object), and returns an error code. Like [lua_call](crate::ffi::lua_callk), lua_pcall always removes the function and its arguments from the stack.
    ///
    /// If msgh is 0, then the error object returned on the stack is exactly the original error object. Otherwise, msgh is the stack index of a message handler. (This index cannot be a pseudo-index.) In case of runtime errors, this handler will be called with the error object and its return value will be the object returned on the stack by [pcall](LuaCore::pcall).
    ///
    /// Typically, the message handler is used to add more debug information to the error object, such as a stack traceback. Such information cannot be gathered after the return of [pcall](LuaCore::pcall), since by then the stack has unwound.
    fn pcall(&self, nargs: i32, nresults: i32, msgh: i32) -> Result<(), LuaError> {
        let result = unsafe {
            lua_pcallk(
                self.get_conn().get_mut_ptr(),
                nargs,
                nresults,
                msgh,
                0,
                None,
            )
        };
        check_for_err!(result);
        Ok(())
    }
    /// Returns the [type](crate::LuaType) of the value in the given valid index.
    fn get_type(&self, index: i32) -> LuaType {
        unsafe {
            let ptr = self.get_conn().get_mut_ptr();
            let ty = lua_type(ptr, index);
            mem::transmute(ty)
        }
    }
    /// Returns the name of the type encoded by the value tp, which must be one the values returned by [get_type](LuaCore::get_type).
    fn type_name(&self, index: i32) -> &str {
        unsafe {
            let ptr = self.get_conn().get_mut_ptr();
            let ptr = lua_typename(ptr, index);
            CStr::from_ptr(ptr).to_str().unwrap()
        }
    }
    /// Pushes onto the stack the value of the global name. Returns the type of that value.
    fn get_global(&self, name: &str) -> LuaType {
        let name = CString::new(name).unwrap();
        unsafe {
            let ty = lua_getglobal(self.get_conn().get_mut_ptr(), name.as_ptr());
            mem::transmute(ty)
        }
    }
    /// Pops a value from the stack and sets it as the new value of global name.
    fn set_global(&self, name: &str) {
        let name = CString::new(name).unwrap();
        unsafe {
            lua_setglobal(self.get_conn().get_mut_ptr(), name.as_ptr());
        }
    }
    /// Pops n elements from the stack.
    fn pop(&self, n: i32) {
        self.set_top(-n - 1);
    }
    ///  Accepts any index, or 0, and sets the stack top to this index. If the new top is greater than the old one, then the new elements are filled with nil. If index is 0, then all stack elements are removed.
    ///
    /// This function can run arbitrary code when removing an index marked as to-be-closed from the stack.
    fn set_top(&self, idx: i32) {
        unsafe {
            lua_settop(self.get_conn().get_mut_ptr(), idx);
        }
    }
    /// Returns the index of the top element in the stack. Because indices start at 1, this result is equal to the number of elements in the stack; in particular, 0 means an empty stack.
    fn get_top(&self) -> i32 {
        unsafe {
            let ptr = self.get_conn().get_mut_ptr();
            lua_gettop(ptr)
        }
    }
    ///  Pushes onto the stack the value t\[k\], where t is the value at the given index and k is the value on the top of the stack.
    ///
    /// This function pops the key from the stack, pushing the resulting value in its place. As in Lua, this function may trigger a metamethod for the "index" event (see §2.4).
    ///
    /// Returns the type of the pushed value.
    fn get_table(&self, idx: i32) -> LuaType {
        unsafe {
            let ty = lua_gettable(self.get_conn().get_mut_ptr(), idx);
            mem::transmute(ty)
        }
    }
    ///  Does the equivalent to t\[k\] = v, where t is the value at the given index, v is the value on the top of the stack, and k is the value just below the top.
    ///
    /// This function pops both the key and the value from the stack. As in Lua, this function may trigger a metamethod for the "newindex" event (see §2.4).
    fn set_table(&self, idx: i32) {
        unsafe {
            lua_settable(self.get_conn().get_mut_ptr(), idx);
        }
    }
    /// Pushes a nil value onto the stack.
    fn push_nil(&self) {
        unsafe {
            lua_pushnil(self.get_conn().get_mut_ptr());
        }
    }
    /// Pushes a boolean value with value b onto the stack.
    fn push_boolean(&self, b: bool) {
        unsafe {
            lua_pushboolean(self.get_conn().get_mut_ptr(), b as i32);
        }
    }
    /// Pushes a float with value n onto the stack.
    fn push_number(&self, n: f64) {
        unsafe {
            lua_pushnumber(self.get_conn().get_mut_ptr(), n);
        }
    }
    ///  Pushes the string pointed to by s with size len onto the stack. Lua will make or reuse an internal copy of the given string, so the memory at s can be freed or reused immediately after the function returns. The string can contain any binary data, including embedded zeros.
    ///
    /// Returns a reference to the internal copy of the string (see §4.1.3).
    fn push_l_string<'a>(&'a self, s: &str, length: isize) -> &'a str {
        let c_string = CString::new(s).unwrap();
        unsafe {
            CStr::from_ptr(lua_pushlstring(
                self.get_conn().get_mut_ptr(),
                c_string.as_ptr(),
                length as usize,
            ))
            .to_str()
            .unwrap()
        }
    }
    ///  Pushes the zero-terminated string pointed to by s onto the stack. Lua will make or reuse an internal copy of the given string, so the memory at s can be freed or reused immediately after the function returns.
    ///
    /// Returns a referetce to the internal copy of the string (see §4.1.3).
    ///
    /// TODO If s is NULL, pushes nil and returns NULL.
    fn push_string<'a>(&'a self, s: &str) -> &'a str {
        let c_string = CString::new(s).unwrap();
        unsafe {
            CStr::from_ptr(lua_pushstring(
                self.get_conn().get_mut_ptr(),
                c_string.as_ptr(),
            ))
            .to_str()
            .unwrap()
        }
    }
    /// Pushes a copy of the element at the given index onto the stack.
    fn push_value(&self, idx: i32) {
        unsafe {
            lua_pushvalue(self.get_conn().get_mut_ptr(), idx);
        }
    }
    /// Pushes a C function onto the stack. This function is equivalent to [lua_pushcclosure] with no upvalues.
    fn push_c_function(&self, f: lua_CFunction) {
        unsafe {
            lua_pushcclosure(self.get_conn().get_mut_ptr(), f, 0);
        }
    }
    /// Pushes value onto stack
    fn push<'a, 'lua, T: Into<LuaStackValue<'a>>>(&'lua self, value: T) -> Option<&'lua str> {
        match value.into() {
            LuaStackValue::Nil => {
                self.push_nil();
                None
            }
            LuaStackValue::Bool(b) => {
                self.push_boolean(b);
                None
            }
            LuaStackValue::Number(n) => {
                self.push_number(n);
                None
            }
            LuaStackValue::String(s) => Some(self.push_string(s)),
            LuaStackValue::CFunction(f) => {
                self.push_c_function(f);
                None
            }
        }
    }
    /// Returns true if the value at the given index is nil, and false otherwise.
    fn is_nil(&self, idx: i32) -> bool {
        matches!(self.get_type(idx), LuaType::Nil)
    }
    /// Returns true if the value at the given index is a boolean, and false otherwise.
    fn is_boolean(&self, idx: i32) -> bool {
        matches!(self.get_type(idx), LuaType::Boolean)
    }
    /// Returns true if the value at the given index is a number or a string convertible to a number, and false otherwise.
    fn is_number(&self, idx: i32) -> bool {
        matches!(self.get_type(idx), LuaType::Number)
    }
    /// Returns true if the value at the given index is a string or a number (which is always convertible to a string), and false otherwise.
    fn is_string(&self, idx: i32) -> bool {
        matches!(self.get_type(idx), LuaType::String)
    }
    /// Returns true if the value at the given index is a table, and false otherwise.
    fn is_table(&self, idx: i32) -> bool {
        matches!(self.get_type(idx), LuaType::Table)
    }
    /// Returns true if the value at the given index is a function (either C or Lua), and true otherwise.
    fn is_function(&self, idx: i32) -> bool {
        matches!(self.get_type(idx), LuaType::Function)
    }
    /// Returns true if the value at the given index is a C function, and false otherwise.
    fn is_c_function(&self, idx: i32) -> bool {
        unsafe {
            match lua_iscfunction(self.get_conn().get_mut_ptr(), idx) {
                0 => false,
                _ => true,
            }
        }
    }
    /// Equivalent to lua_tolstring with len equal to NULL.
    fn to_string(&self, idx: i32) -> &str {
        unsafe {
            let ptr = self.get_conn().get_mut_ptr();
            let ptr = lua_tolstring(ptr, idx, std::ptr::null_mut());
            CStr::from_ptr(ptr).to_str().unwrap()
        }
    }
    /// Equivalent to lua_tonumberx with isnum equal to NULL.
    fn to_number(&self, idx: i32) -> f64 {
        unsafe { lua_tonumberx(self.get_conn().get_mut_ptr(), idx, std::ptr::null_mut()) }
    }
    ///  Converts the Lua value at the given index to a C boolean value (0 or 1). Like all tests in Lua, lua_toboolean returns true for any Lua value different from false and nil; otherwise it returns false. (If you want to accept only actual boolean values, use lua_isboolean to test the value's type.)
    fn to_boolean(&self, idx: i32) -> bool {
        unsafe {
            match lua_toboolean(self.get_conn().get_mut_ptr(), idx) {
                0 => false,
                _ => true,
            }
        }
    }
    /// Ensures that the stack has space for at least n extra elements, that is, that you can safely push up to n values into it. It returns false if it cannot fulfill the request, either because it would cause the stack to be greater than a fixed maximum size (typically at least several thousand elements) or because it cannot allocate memory for the extra space. This function never shrinks the stack; if the stack already has space for the extra elements, it is left unchanged.
    fn check_stack(&self, sz: i32) -> bool {
        unsafe {
            match lua_checkstack(self.get_conn().get_mut_ptr(), sz) {
                0 => false,
                _ => true,
            }
        }
    }
    /// Removes the element at the given valid index, shifting down the elements above this index to fill the gap. This function cannot be called with a pseudo-index, because a pseudo-index is not an actual stack position.
    fn remove(&self, idx: i32) {
        self.rotate(idx, -1);
        self.pop(1)
    }
    /// Moves the top element into the given valid index, shifting up the elements above this index to open space. This function cannot be called with a pseudo-index, because a pseudo-index is not an actual stack position.
    fn insert(&self, idx: i32) {
        self.rotate(idx, 1);
    }
    /// Moves the top element into the given valid index without shifting any element (therefore replacing the value at that given index), and then pops the top element.
    fn replace(&self, idx: i32) {
        self.copy(-1, idx);
        self.pop(1);
    }
    /// Copies the element at index fromidx into the valid index toidx, replacing the value at that position. Values at other positions are not affected.
    fn copy(&self, from_idx: i32, to_idx: i32) {
        unsafe {
            lua_copy(self.get_conn().get_mut_ptr(), from_idx, to_idx);
        }
    }
    /// Rotates the stack elements between the valid index idx and the top of the stack. The elements are rotated n positions in the direction of the top, for a positive n, or -n positions in the direction of the bottom, for a negative n. The absolute value of n must not be greater than the size of the slice being rotated. This function cannot be called with a pseudo-index, because a pseudo-index is not an actual stack position.
    fn rotate(&self, idx: i32, n: i32) {
        unsafe {
            lua_rotate(self.get_conn().get_mut_ptr(), idx, n);
        }
    }
    /// Raises a Lua error, using the value on the top of the stack as the error object. This function does a long jump, and therefore never returns (see [aux_error](crate::LuaAuxLib::aux_error)).
    fn error(&self) {
        unsafe {
            lua_error(self.get_conn().get_mut_ptr());
        }
    }
}
impl<T: LuaConn> LuaCore for T {}

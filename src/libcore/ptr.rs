//! Unsafe pointer utility functions

export addr_of;
export to_unsafe_ptr;
export to_const_unsafe_ptr;
export to_mut_unsafe_ptr;
export mut_addr_of;
export offset;
export const_offset;
export mut_offset;
export null;
export is_null;
export is_not_null;
export memcpy;
export memmove;
export memset;
export to_uint;
export ref_eq;
export buf_len;
export position;
export Ptr;

use cmp::{Eq, Ord};
use libc::{c_void, size_t};

#[nolink]
#[abi = "cdecl"]
extern mod libc_ {
    #[rust_stack]
    fn memcpy(dest: *mut c_void, src: *const c_void,
              n: libc::size_t) -> *c_void;

    #[rust_stack]
    fn memmove(dest: *mut c_void, src: *const c_void,
               n: libc::size_t) -> *c_void;

    #[rust_stack]
    fn memset(dest: *mut c_void, c: libc::c_int,
              len: libc::size_t) -> *c_void;
}

#[abi = "rust-intrinsic"]
extern mod rusti {
    fn addr_of<T>(val: T) -> *T;
}

/// Get an unsafe pointer to a value
#[inline(always)]
pure fn addr_of<T>(val: T) -> *T { unchecked { rusti::addr_of(val) } }

/// Get an unsafe mut pointer to a value
#[inline(always)]
pure fn mut_addr_of<T>(val: T) -> *mut T {
    unsafe {
        unsafe::reinterpret_cast(&rusti::addr_of(val))
    }
}

/// Calculate the offset from a pointer
#[inline(always)]
fn offset<T>(ptr: *T, count: uint) -> *T {
    unsafe {
        (ptr as uint + count * sys::size_of::<T>()) as *T
    }
}

/// Calculate the offset from a const pointer
#[inline(always)]
fn const_offset<T>(ptr: *const T, count: uint) -> *const T {
    unsafe {
        (ptr as uint + count * sys::size_of::<T>()) as *T
    }
}

/// Calculate the offset from a mut pointer
#[inline(always)]
fn mut_offset<T>(ptr: *mut T, count: uint) -> *mut T {
    (ptr as uint + count * sys::size_of::<T>()) as *mut T
}

/// Return the offset of the first null pointer in `buf`.
#[inline(always)]
unsafe fn buf_len<T>(buf: **T) -> uint {
    position(buf, |i| i == null())
}

/// Return the first offset `i` such that `f(buf[i]) == true`.
#[inline(always)]
unsafe fn position<T>(buf: *T, f: fn(T) -> bool) -> uint {
    let mut i = 0u;
    loop {
        if f(*offset(buf, i)) { return i; }
        else { i += 1u; }
    }
}

/// Create an unsafe null pointer
#[inline(always)]
pure fn null<T>() -> *T { unsafe { unsafe::reinterpret_cast(&0u) } }

/// Returns true if the pointer is equal to the null pointer.
pure fn is_null<T>(ptr: *const T) -> bool { ptr == null() }

/// Returns true if the pointer is not equal to the null pointer.
pure fn is_not_null<T>(ptr: *const T) -> bool { !is_null(ptr) }

/**
 * Copies data from one location to another
 *
 * Copies `count` elements (not bytes) from `src` to `dst`. The source
 * and destination may not overlap.
 */
#[inline(always)]
unsafe fn memcpy<T>(dst: *mut T, src: *const T, count: uint) {
    let n = count * sys::size_of::<T>();
    libc_::memcpy(dst as *mut c_void, src as *c_void, n as size_t);
}

/**
 * Copies data from one location to another
 *
 * Copies `count` elements (not bytes) from `src` to `dst`. The source
 * and destination may overlap.
 */
#[inline(always)]
unsafe fn memmove<T>(dst: *mut T, src: *const T, count: uint)  {
    let n = count * sys::size_of::<T>();
    libc_::memmove(dst as *mut c_void, src as *c_void, n as size_t);
}

#[inline(always)]
unsafe fn memset<T>(dst: *mut T, c: int, count: uint)  {
    let n = count * sys::size_of::<T>();
    libc_::memset(dst as *mut c_void, c as libc::c_int, n as size_t);
}


/**
  Transform a region pointer - &T - to an unsafe pointer - *T.
  This is safe, but is implemented with an unsafe block due to
  reinterpret_cast.
*/
#[inline(always)]
fn to_unsafe_ptr<T>(thing: &T) -> *T {
    unsafe { unsafe::reinterpret_cast(&thing) }
}

/**
  Transform a const region pointer - &const T - to a const unsafe pointer -
  *const T. This is safe, but is implemented with an unsafe block due to
  reinterpret_cast.
*/
#[inline(always)]
fn to_const_unsafe_ptr<T>(thing: &const T) -> *const T {
    unsafe { unsafe::reinterpret_cast(&thing) }
}

/**
  Transform a mutable region pointer - &mut T - to a mutable unsafe pointer -
  *mut T. This is safe, but is implemented with an unsafe block due to
  reinterpret_cast.
*/
#[inline(always)]
fn to_mut_unsafe_ptr<T>(thing: &mut T) -> *mut T {
    unsafe { unsafe::reinterpret_cast(&thing) }
}

/**
  Cast a region pointer - &T - to a uint.
  This is safe, but is implemented with an unsafe block due to
  reinterpret_cast.

  (I couldn't think of a cutesy name for this one.)
*/
#[inline(always)]
fn to_uint<T>(thing: &T) -> uint unsafe {
    unsafe::reinterpret_cast(&thing)
}

/// Determine if two borrowed pointers point to the same thing.
#[inline(always)]
fn ref_eq<T>(thing: &a/T, other: &b/T) -> bool {
    to_uint(thing) == to_uint(other)
}

trait Ptr {
    pure fn is_null() -> bool;
    pure fn is_not_null() -> bool;
}

/// Extension methods for pointers
impl<T> *T: Ptr {
    /// Returns true if the pointer is equal to the null pointer.
    pure fn is_null() -> bool { is_null(self) }

    /// Returns true if the pointer is not equal to the null pointer.
    pure fn is_not_null() -> bool { is_not_null(self) }
}

// Equality for pointers
impl<T> *const T : Eq {
    pure fn eq(&&other: *const T) -> bool unsafe {
        let a: uint = unsafe::reinterpret_cast(&self);
        let b: uint = unsafe::reinterpret_cast(&other);
        return a == b;
    }
    pure fn ne(&&other: *const T) -> bool { !self.eq(other) }
}

// Comparison for pointers
impl<T> *const T : Ord {
    pure fn lt(&&other: *const T) -> bool unsafe {
        let a: uint = unsafe::reinterpret_cast(&self);
        let b: uint = unsafe::reinterpret_cast(&other);
        return a < b;
    }
    pure fn le(&&other: *const T) -> bool unsafe {
        let a: uint = unsafe::reinterpret_cast(&self);
        let b: uint = unsafe::reinterpret_cast(&other);
        return a <= b;
    }
    pure fn ge(&&other: *const T) -> bool unsafe {
        let a: uint = unsafe::reinterpret_cast(&self);
        let b: uint = unsafe::reinterpret_cast(&other);
        return a >= b;
    }
    pure fn gt(&&other: *const T) -> bool unsafe {
        let a: uint = unsafe::reinterpret_cast(&self);
        let b: uint = unsafe::reinterpret_cast(&other);
        return a > b;
    }
}

// Equality for region pointers
impl<T:Eq> &const T : Eq {
    pure fn eq(&&other: &const T) -> bool { return *self == *other; }
    pure fn ne(&&other: &const T) -> bool { return *self != *other; }
}

// Comparison for region pointers
impl<T:Ord> &const T : Ord {
    pure fn lt(&&other: &const T) -> bool { *self < *other }
    pure fn le(&&other: &const T) -> bool { *self <= *other }
    pure fn ge(&&other: &const T) -> bool { *self >= *other }
    pure fn gt(&&other: &const T) -> bool { *self > *other }
}

#[test]
fn test() {
    unsafe {
        type Pair = {mut fst: int, mut snd: int};
        let p = {mut fst: 10, mut snd: 20};
        let pptr: *mut Pair = mut_addr_of(p);
        let iptr: *mut int = unsafe::reinterpret_cast(&pptr);
        assert (*iptr == 10);;
        *iptr = 30;
        assert (*iptr == 30);
        assert (p.fst == 30);;

        *pptr = {mut fst: 50, mut snd: 60};
        assert (*iptr == 50);
        assert (p.fst == 50);
        assert (p.snd == 60);

        let mut v0 = ~[32000u16, 32001u16, 32002u16];
        let mut v1 = ~[0u16, 0u16, 0u16];

        ptr::memcpy(ptr::mut_offset(vec::unsafe::to_mut_ptr(v1), 1u),
                    ptr::offset(vec::unsafe::to_ptr(v0), 1u), 1u);
        assert (v1[0] == 0u16 && v1[1] == 32001u16 && v1[2] == 0u16);
        ptr::memcpy(vec::unsafe::to_mut_ptr(v1),
                    ptr::offset(vec::unsafe::to_ptr(v0), 2u), 1u);
        assert (v1[0] == 32002u16 && v1[1] == 32001u16 && v1[2] == 0u16);
        ptr::memcpy(ptr::mut_offset(vec::unsafe::to_mut_ptr(v1), 2u),
                    vec::unsafe::to_ptr(v0), 1u);
        assert (v1[0] == 32002u16 && v1[1] == 32001u16 && v1[2] == 32000u16);
    }
}

#[test]
fn test_position() {
    use str::as_c_str;
    use libc::c_char;

    let s = ~"hello";
    unsafe {
        assert 2u == as_c_str(s, |p| position(p, |c| c == 'l' as c_char));
        assert 4u == as_c_str(s, |p| position(p, |c| c == 'o' as c_char));
        assert 5u == as_c_str(s, |p| position(p, |c| c == 0 as c_char));
    }
}

#[test]
fn test_buf_len() {
    let s0 = ~"hello";
    let s1 = ~"there";
    let s2 = ~"thing";
    do str::as_c_str(s0) |p0| {
        do str::as_c_str(s1) |p1| {
            do str::as_c_str(s2) |p2| {
                let v = ~[p0, p1, p2, null()];
                do vec::as_buf(v) |vp, len| {
                    assert unsafe { buf_len(vp) } == 3u;
                    assert len == 4u;
                }
            }
        }
    }
}

#[test]
fn test_is_null() {
   let p: *int = ptr::null();
   assert p.is_null();
   assert !p.is_not_null();

   let q = ptr::offset(p, 1u);
   assert !q.is_null();
   assert q.is_not_null();
}

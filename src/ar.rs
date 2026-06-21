use libc::{fnmatch, free, strchr};

pub use crate::ffi_types::{__time_t, intmax_t, size_t, time_t, uintmax_t};
use crate::file::{Dep, File};
use crate::misc::{xcalloc, xstrdup};
use crate::strcache::strcache_add;
extern "C" {
    pub type variable_set_list;
    pub type commands;
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type file = File;
pub type cmd_state = ::core::ffi::c_uint;
pub const cs_finished: cmd_state = 3;
pub const cs_running: cmd_state = 2;
pub const cs_deps_running: cmd_state = 1;
pub const cs_not_started: cmd_state = 0;
pub type update_status = ::core::ffi::c_uint;
pub type update_status_0 = u32;
pub const us_failed: update_status_0 = 3;
pub const us_question: update_status_0 = 2;
pub const us_none: update_status_0 = 1;
pub const us_success: update_status_0 = 0;
pub type dep = Dep;
use crate::floc::Floc;

pub type ar_member_func_t = Option<
    unsafe fn(
        ::core::ffi::c_int,
        *const ::core::ffi::c_char,
        ::core::ffi::c_int,
        ::core::ffi::c_long,
        ::core::ffi::c_long,
        ::core::ffi::c_long,
        intmax_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_uint,
        *const ::core::ffi::c_void,
    ) -> intmax_t,
>;
use crate::arscan::{ar_member_touch, ar_name_equal, ar_scan};
use crate::dir::file_exists_p;
pub use crate::file::nameseq;
use crate::file::{enter_file, lookup_file};
use crate::misc::{alpha_compare, concat};
use crate::output::{error, fatal, out_of_memory, perror_with_name};
use crate::remake::f_mtime;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ar_glob_state {
    pub arname: *const ::core::ffi::c_char,
    pub pattern: *const ::core::ffi::c_char,
    pub size: size_t,
    pub chain: *mut nameseq,
    pub n: ::core::ffi::c_uint,
}
pub const CHAR_BIT: ::core::ffi::c_int = __CHAR_BIT__;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FNM_PATHNAME: ::core::ffi::c_int = (1) << 0;
pub const FNM_PERIOD: ::core::ffi::c_int = (1) << 2;
/// Classification of a target name with respect to the `archive(member)`
/// syntax recognized by [`ar_name`].
enum ArName {
    /// Not an archive reference: no usable `(member)` suffix.
    Plain,
    /// A well-formed `archive(member)` reference.
    Member,
    /// The unsupported nested `archive((member))` form.
    Unsupported,
}

/// Classify `bytes` (a target name, without its terminating NUL) as an archive
/// reference. Pure mirror of make's `ar_name` parsing logic.
fn classify_ar_name(bytes: &[u8]) -> ArName {
    // Find the first '('; it must exist and not be the very first byte.
    let lp = match bytes.iter().position(|&c| c == b'(') {
        None | Some(0) => return ArName::Plain,
        Some(i) => i,
    };
    // The name must end with ')', and the member must be non-empty (the ')'
    // cannot sit immediately after the '(').
    let last = bytes.len() - 1;
    if bytes[last] != b')' || last == lp + 1 {
        return ArName::Plain;
    }
    // `archive((member))` is the unsupported nested form.
    if bytes[lp + 1] == b'(' && bytes[last - 1] == b')' {
        return ArName::Unsupported;
    }
    ArName::Member
}

/// Does `name` refer to an `archive(member)` target?
///
/// Aborts via [`fatal`] on the unsupported nested `archive((member))` form,
/// matching make's behavior.
pub fn ar_name(name: &::core::ffi::CStr) -> bool {
    match classify_ar_name(name.to_bytes()) {
        ArName::Plain => false,
        ArName::Member => true,
        ArName::Unsupported => unsafe {
            fatal(
                ::core::ptr::null_mut::<Floc>(),
                name.to_bytes().len() as size_t,
                b"attempt to use unsupported feature: '%s'\0" as *const u8
                    as *const ::core::ffi::c_char,
                name.as_ptr(),
            )
        },
    }
}
/// # Safety
///
/// C-style API operating on raw pointers; all pointer arguments must be
/// valid (NUL-terminated where strings are expected) for the call.
pub unsafe fn ar_parse_name(
    name: *const ::core::ffi::c_char,
    arname_p: *mut *mut ::core::ffi::c_char,
    memname_p: *mut *mut ::core::ffi::c_char,
) {
    let mut p: *mut ::core::ffi::c_char;
    *arname_p = xstrdup(name);
    p = strchr(*arname_p, '(' as i32);
    if p.is_null() {
        fatal(
            ::core::ptr::null_mut::<Floc>(),
            strlen(*arname_p) as size_t,
            b"INTERNAL: ar_parse_name: bad name '%s'\0" as *const u8 as *const ::core::ffi::c_char,
            *arname_p,
        );
    }
    let fresh0 = p;
    p = p.offset(1 as ::core::ffi::c_int as isize);
    *fresh0 = 0;
    *p.offset(strlen(p).wrapping_sub(1) as isize) = 0;
    *memname_p = p;
}
// The argument list is the fixed ar_scan callback protocol.
#[allow(clippy::too_many_arguments)]
unsafe fn ar_member_date_1(
    mut _desc: ::core::ffi::c_int,
    mem: *const ::core::ffi::c_char,
    truncated: ::core::ffi::c_int,
    mut _hdrpos: ::core::ffi::c_long,
    mut _datapos: ::core::ffi::c_long,
    mut _size: ::core::ffi::c_long,
    date: intmax_t,
    mut _uid: ::core::ffi::c_int,
    mut _gid: ::core::ffi::c_int,
    mut _mode: ::core::ffi::c_uint,
    name: *const ::core::ffi::c_void,
) -> intmax_t {
    if ar_name_equal(name as *const ::core::ffi::c_char, mem, truncated) != 0 {
        date
    } else {
        0 as intmax_t
    }
}
/// Owns a parsed `archive(member)` name split into two NUL-terminated C
/// strings inside a single buffer.
///
/// `ar_parse_name` historically did `xstrdup(name)`, overwrote the `(` and the
/// trailing `)` with NULs in place, and handed the caller back two interior
/// pointers it then had to `free`. `ParsedArName` replaces that `xstrdup`/`free`
/// ownership pair with an owned `Vec<u8>` that drops automatically: it holds
/// `archive\0member\0`, so [`arname`](Self::arname) is the leading C string and
/// [`memname`](Self::memname) starts at `member_off`. Nothing escapes — the
/// buffer is allocated, used, and dropped entirely within the calling function.
struct ParsedArName {
    /// The dup of `name` with `(` and the trailing `)` rewritten as NULs,
    /// i.e. `archive\0member\0`, kept owned so it drops at end of scope.
    buf: Vec<u8>,
    /// Byte offset of the member C string within `buf`.
    member_off: usize,
}

impl ParsedArName {
    /// Parse `name` (a well-formed `archive(member)` reference, as guaranteed by
    /// a prior [`ar_name`] check) into an owned buffer. Mirrors `ar_parse_name`:
    /// split at the first `(`, then drop the trailing `)`.
    ///
    /// # Safety
    ///
    /// May call [`out_of_memory`], which is `unsafe`, on allocation failure.
    /// Marked `unsafe` so the `out_of_memory()` diagnostic path is reachable
    /// without introducing a new `unsafe` block.
    unsafe fn parse(name: &::core::ffi::CStr) -> Self {
        let src = name.to_bytes_with_nul();
        // Reserve fallibly so OOM routes through make's `out_of_memory()`
        // ("virtual memory exhausted") diagnostic, matching the original
        // `xstrdup`, rather than aborting via Rust's allocation-error path.
        let mut buf = Vec::new();
        if buf.try_reserve_exact(src.len()).is_err() {
            out_of_memory();
        }
        buf.extend_from_slice(src);
        // `ar_name` guarantees a '(' exists and the name ends with ')'.
        let lp = buf
            .iter()
            .position(|&c| c == b'(')
            .expect("ParsedArName::parse: ar_name guarantees a '('");
        buf[lp] = 0;
        let member_off = lp + 1;
        // Overwrite the trailing ')' (the byte before the original NUL) with a
        // NUL, exactly as `p[strlen(p) - 1] = '\0'` did.
        let close = buf.len() - 2;
        buf[close] = 0;
        ParsedArName { buf, member_off }
    }

    /// The archive C string (`archive`).
    fn arname(&self) -> *const ::core::ffi::c_char {
        self.buf.as_ptr() as *const ::core::ffi::c_char
    }

    /// The member C string (`member`).
    fn memname(&self) -> *const ::core::ffi::c_char {
        self.buf[self.member_off..].as_ptr() as *const ::core::ffi::c_char
    }
}

/// # Safety
///
/// C-style API operating on raw pointers; all pointer arguments must be
/// valid (NUL-terminated where strings are expected) for the call.
pub unsafe fn ar_member_date(name: *const ::core::ffi::c_char) -> time_t {
    // `name` is `archive(member)`; own the split buffer here so it drops on
    // return (replacing the old `ar_parse_name` xstrdup + `free`).
    let parsed = ParsedArName::parse(::core::ffi::CStr::from_ptr(name));
    let arname = parsed.arname();
    let memname = parsed.memname();
    let val: intmax_t;
    let mut arfile: *mut file;
    arfile = lookup_file(arname);
    if arfile.is_null() && file_exists_p(arname) != 0 {
        arfile = enter_file(strcache_add(arname));
    }
    if !arfile.is_null() {
        f_mtime(arfile, 0);
    }
    val = ar_scan(
        arname,
        Some(ar_member_date_1),
        memname as *const ::core::ffi::c_void,
    );
    if (0 as intmax_t) < val
        && val
            <= (if (0 as ::core::ffi::c_int as time_t) < -(1 as ::core::ffi::c_int) as time_t {
                -(1 as ::core::ffi::c_int) as time_t
            } else {
                (((1 as ::core::ffi::c_int as time_t)
                    << (::core::mem::size_of::<time_t>() as usize)
                        .wrapping_mul(CHAR_BIT as usize)
                        .wrapping_sub(2 as usize))
                    - 1 as time_t)
                    * 2 as time_t
                    + 1 as time_t
            }) as intmax_t
    {
        val as time_t
    } else {
        -(1 as ::core::ffi::c_int) as time_t
    }
}
/// # Safety
///
/// C-style API operating on raw pointers; all pointer arguments must be
/// valid (NUL-terminated where strings are expected) for the call.
pub unsafe fn ar_touch(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut arname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut memname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut val: ::core::ffi::c_int;
    ar_parse_name(name, &raw mut arname, &raw mut memname);
    let arfile: *mut file;
    arfile = enter_file(strcache_add(arname));
    f_mtime(arfile, 0);
    val = 1;
    match ar_member_touch(arname, memname) {
        -1 => {
            error(
                ::core::ptr::null_mut::<Floc>(),
                strlen(arname) as size_t,
                b"touch: archive '%s' does not exist\0" as *const u8 as *const ::core::ffi::c_char,
                arname,
            );
        }
        -2 => {
            error(
                ::core::ptr::null_mut::<Floc>(),
                strlen(arname) as size_t,
                b"touch: '%s' is not a valid archive\0" as *const u8 as *const ::core::ffi::c_char,
                arname,
            );
        }
        -3 => {
            perror_with_name(
                b"touch: \0" as *const u8 as *const ::core::ffi::c_char,
                arname,
            );
        }
        1 => {
            error(
                ::core::ptr::null_mut::<Floc>(),
                (strlen(memname) as size_t).wrapping_add(strlen(arname) as size_t),
                b"touch: member '%s' does not exist in '%s'\0" as *const u8
                    as *const ::core::ffi::c_char,
                memname,
                arname,
            );
        }
        0 => {
            val = 0;
        }
        _ => {
            error(
                ::core::ptr::null_mut::<Floc>(),
                strlen(name) as size_t,
                b"touch: bad return code from ar_member_touch on '%s'\0" as *const u8
                    as *const ::core::ffi::c_char,
                name,
            );
        }
    }
    free(arname as *mut ::core::ffi::c_void);
    val
}
// The argument list is the fixed ar_scan callback protocol.
#[allow(clippy::too_many_arguments)]
unsafe fn ar_glob_match(
    mut _desc: ::core::ffi::c_int,
    mem: *const ::core::ffi::c_char,
    mut _truncated: ::core::ffi::c_int,
    mut _hdrpos: ::core::ffi::c_long,
    mut _datapos: ::core::ffi::c_long,
    mut _size: ::core::ffi::c_long,
    mut _date: intmax_t,
    mut _uid: ::core::ffi::c_int,
    mut _gid: ::core::ffi::c_int,
    mut _mode: ::core::ffi::c_uint,
    arg: *const ::core::ffi::c_void,
) -> intmax_t {
    let state: *mut ar_glob_state = arg as *mut ar_glob_state;
    if fnmatch((*state).pattern, mem, FNM_PATHNAME | FNM_PERIOD) == 0 {
        let new: *mut nameseq = xcalloc((*state).size) as *mut nameseq;
        (*new).name = strcache_add(concat(
            4,
            (*state).arname,
            b"(\0" as *const u8 as *const ::core::ffi::c_char,
            mem,
            b")\0" as *const u8 as *const ::core::ffi::c_char,
        ));
        (*new).next = (*state).chain;
        (*state).chain = new;
        (*state).n = (*state).n.wrapping_add(1);
    }
    0 as intmax_t
}
/// Does `pattern` contain shell glob metacharacters (`?`, `*`, or a balanced
/// `[`…`]`)? When `quote` is set, a backslash escapes the following byte, so it
/// is skipped rather than treated as a metacharacter. Pure mirror of make's
/// `ar_glob_pattern_p`.
fn ar_glob_pattern_p(pattern: &[u8], quote: bool) -> bool {
    let mut opened = false;
    let mut i = 0;
    while i < pattern.len() {
        match pattern[i] {
            b'?' | b'*' => return true,
            // Skip the escaped byte; bounded indexing avoids the C version's
            // read past the NUL when a trailing backslash is escaped.
            b'\\' if quote => i += 1,
            b'[' => opened = true,
            b']' if opened => return true,
            _ => {}
        }
        i += 1;
    }
    false
}

#[cfg(test)]
mod ar_glob_pattern_p_tests {
    use super::ar_glob_pattern_p;

    #[test]
    fn detects_glob_metacharacters() {
        assert!(ar_glob_pattern_p(b"*.o", true));
        assert!(ar_glob_pattern_p(b"foo?", true));
        assert!(ar_glob_pattern_p(b"foo[abc].o", true));
    }

    #[test]
    fn plain_names_are_not_patterns() {
        assert!(!ar_glob_pattern_p(b"", true));
        assert!(!ar_glob_pattern_p(b"foo.o", true));
        // A ']' without a preceding '[' is not a class.
        assert!(!ar_glob_pattern_p(b"foo].o", true));
    }

    #[test]
    fn backslash_quoting() {
        // With quoting on, the escaped '*' is consumed, not matched.
        assert!(!ar_glob_pattern_p(b"foo\\*", true));
        // With quoting off, the backslash is inert and the '*' still matches.
        assert!(ar_glob_pattern_p(b"foo\\*", false));
        // A trailing backslash must not read past the end of the slice.
        assert!(!ar_glob_pattern_p(b"foo\\", true));
    }
}
/// # Safety
///
/// C-style API operating on raw pointers; all pointer arguments must be
/// valid (NUL-terminated where strings are expected) for the call.
pub unsafe fn ar_glob(
    arname: *const ::core::ffi::c_char,
    member_pattern: *const ::core::ffi::c_char,
    size: size_t,
) -> *mut nameseq {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut state: ar_glob_state = ar_glob_state {
        arname: ::core::ptr::null::<::core::ffi::c_char>(),
        pattern: ::core::ptr::null::<::core::ffi::c_char>(),
        size: 0,
        chain: ::core::ptr::null_mut::<nameseq>(),
        n: 0,
    };
    let mut n: *mut nameseq;
    let names: *mut *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_uint;
    if !ar_glob_pattern_p(::core::ffi::CStr::from_ptr(member_pattern).to_bytes(), true) {
        return ::core::ptr::null_mut::<nameseq>();
    }
    state.arname = arname;
    state.pattern = member_pattern;
    state.size = size;
    state.chain = ::core::ptr::null_mut::<nameseq>();
    state.n = 0;
    ar_scan(
        arname,
        Some(ar_glob_match),
        &raw mut state as *const ::core::ffi::c_void,
    );
    if state.chain.is_null() {
        return ::core::ptr::null_mut::<nameseq>();
    }
    alloca_allocations.push(::std::vec::from_elem(
        0,
        (state.n as usize)
            .wrapping_mul(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
            as usize,
    ));
    names = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut *const ::core::ffi::c_char;
    i = 0;
    n = state.chain;
    while let Some(nref) = n.as_mut() {
        let fresh1 = i;
        i = i.wrapping_add(1);
        let fresh2 = &mut (*names.offset(fresh1 as isize));
        *fresh2 = nref.name;
        n = nref.next;
    }
    qsort(
        names as *mut ::core::ffi::c_void,
        i as size_t,
        ::core::mem::size_of::<*const ::core::ffi::c_char>() as size_t,
        Some(alpha_compare),
    );
    i = 0;
    n = state.chain;
    while let Some(nref) = n.as_mut() {
        let fresh3 = i;
        i = i.wrapping_add(1);
        nref.name = *names.offset(fresh3 as isize);
        n = nref.next;
    }
    state.chain
}
pub const __CHAR_BIT__: ::core::ffi::c_int = 8;

#[cfg(test)]
mod ar_name_tests {
    use super::{classify_ar_name, ArName};

    fn kind(s: &str) -> &'static str {
        match classify_ar_name(s.as_bytes()) {
            ArName::Plain => "plain",
            ArName::Member => "member",
            ArName::Unsupported => "unsupported",
        }
    }

    #[test]
    fn plain_names_are_not_archives() {
        assert_eq!(kind("foo.o"), "plain"); // no parenthesis
        assert_eq!(kind(""), "plain"); // empty
        assert_eq!(kind("(member)"), "plain"); // '(' at the very start
        assert_eq!(kind("lib("), "plain"); // no closing ')'
        assert_eq!(kind("lib(member"), "plain"); // missing ')'
        assert_eq!(kind("lib()"), "plain"); // empty member
        assert_eq!(kind("lib(member)x"), "plain"); // ')' is not the last byte
    }

    #[test]
    fn well_formed_archive_members() {
        assert_eq!(kind("lib.a(member.o)"), "member");
        assert_eq!(kind("lib(m)"), "member");
        assert_eq!(kind("a(bc)"), "member");
    }

    #[test]
    fn nested_form_is_unsupported() {
        assert_eq!(kind("lib((member))"), "unsupported");
        assert_eq!(kind("a((b))"), "unsupported");
    }

    #[test]
    fn single_inner_open_paren_is_a_member() {
        // Only the unsupported form needs both an inner '(' and a matching
        // inner ')'. A lone inner '(' is treated as part of the member name.
        assert_eq!(kind("lib((member)"), "member");
    }
}

#[cfg(test)]
mod parsed_ar_name_tests {
    use super::ParsedArName;

    /// Pre-conversion oracle: the in-place split that `ar_parse_name` did on an
    /// `xstrdup`'d copy. Operates on an owned `Vec<u8>` (mirroring the dup),
    /// overwrites the first `(` and the trailing `)` with NULs, and returns the
    /// resulting `(arname, memname)` C-string byte slices (without their NULs),
    /// reproducing the interior pointers `ar_parse_name` handed back.
    fn ar_parse_name_oracle(name: &[u8]) -> (Vec<u8>, Vec<u8>) {
        // xstrdup: a NUL-terminated copy.
        let mut buf = name.to_vec();
        buf.push(0);
        // p = strchr(arname, '('); *(p++) = '\0';
        let lp = buf
            .iter()
            .position(|&c| c == b'(')
            .expect("oracle: name must contain '('");
        buf[lp] = 0;
        // p[strlen(p) - 1] = '\0';  (the trailing ')' before the copy's NUL).
        let close = buf.len() - 2;
        buf[close] = 0;
        let arname = buf[..lp].to_vec();
        let memname = buf[lp + 1..close].to_vec();
        (arname, memname)
    }

    /// Drive the safe `ParsedArName` and the preserved oracle through the same
    /// inputs and assert byte-identical archive/member strings, including
    /// embedded NUL-adjacent, high-byte, and single-character members.
    fn assert_same(name: &[u8]) {
        let cs = ::std::ffi::CString::new(name).expect("test input has no embedded NUL");
        let parsed = unsafe { ParsedArName::parse(&cs) };
        // Read the produced C strings back as byte slices (no terminating NUL).
        let got_ar = unsafe { ::core::ffi::CStr::from_ptr(parsed.arname()) }.to_bytes();
        let got_mem = unsafe { ::core::ffi::CStr::from_ptr(parsed.memname()) }.to_bytes();
        let (want_ar, want_mem) = ar_parse_name_oracle(name);
        assert_eq!(got_ar, &want_ar[..], "arname mismatch for {name:?}");
        assert_eq!(got_mem, &want_mem[..], "memname mismatch for {name:?}");
    }

    #[test]
    fn matches_oracle_on_representative_inputs() {
        assert_same(b"lib.a(member.o)");
        assert_same(b"a(b)"); // single-char archive and member
        assert_same(b"lib((member)"); // lone inner '(' is part of the member
        assert_same(b"/path/to/lib.a(very_long_member_name.o)");
        assert_same(b"\xc3\xa9lib.a(m\xc3\xa9mber.o)"); // high bytes both sides
        assert_same(b"x(\xff)"); // high-byte single-byte member
    }
}

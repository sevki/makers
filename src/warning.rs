use crate::floc::Floc;

extern "C" {
    fn error(flocp: *const Floc, length: usize, fmt: *const ::core::ffi::c_char, ...);
    fn fatal(flocp: *const Floc, length: usize, fmt: *const ::core::ffi::c_char, ...) -> !;
    fn variable_buffer_output(
        ptr: *mut ::core::ffi::c_char,
        string: *const ::core::ffi::c_char,
        length: usize,
    ) -> *mut ::core::ffi::c_char;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Action {
    #[default]
    Unset = 0,
    Ignore = 1,
    Warn = 2,
    Error = 3,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Type {
    CircularDep = 0,
    InvalidRef = 1,
    InvalidVar = 2,
    UndefinedVar = 3,
}

impl Type {
    pub const COUNT: usize = 4;
    const ALL: [Type; Self::COUNT] = [
        Type::CircularDep,
        Type::InvalidRef,
        Type::InvalidVar,
        Type::UndefinedVar,
    ];
    fn name(self) -> &'static str {
        match self {
            Type::CircularDep => "circular-dep",
            Type::InvalidRef => "invalid-ref",
            Type::InvalidVar => "invalid-var",
            Type::UndefinedVar => "undefined-var",
        }
    }
    fn from_name(s: &str) -> Option<Type> {
        Self::ALL.into_iter().find(|t| t.name().eq_ignore_ascii_case(s))
    }
}

impl Action {
    fn name(self) -> Option<&'static str> {
        match self {
            Action::Unset => None,
            Action::Ignore => Some("ignore"),
            Action::Warn => Some("warn"),
            Action::Error => Some("error"),
        }
    }
    fn from_name(s: &str) -> Option<Action> {
        for a in [Action::Ignore, Action::Warn, Action::Error] {
            if a.name().unwrap().eq_ignore_ascii_case(s) {
                return Some(a);
            }
        }
        None
    }
}

#[derive(Default, Copy, Clone)]
pub struct Data {
    pub global: Action,
    pub actions: [Action; Type::COUNT],
}

/// Active per-warning action, indexed by `Type as usize`. Callers index into
/// this directly when checking whether a warning is enabled.
#[no_mangle]
pub static mut warnings: [Action; Type::COUNT] = [Action::Unset; Type::COUNT];

static mut WARN_DEFAULT: Data = Data {
    global: Action::Unset,
    actions: [Action::Unset; Type::COUNT],
};
static mut WARN_VARIABLE: Data = Data {
    global: Action::Unset,
    actions: [Action::Unset; Type::COUNT],
};
static mut WARN_FLAG: Data = Data {
    global: Action::Unset,
    actions: [Action::Unset; Type::COUNT],
};

/// Resolve the active per-warning action by walking the precedence chain:
/// per-flag → flag-global → per-variable → variable-global → default.
fn refresh_warnings() {
    unsafe {
        for t in Type::ALL {
            let i = t as usize;
            warnings[i] = if WARN_FLAG.actions[i] != Action::Unset {
                WARN_FLAG.actions[i]
            } else if WARN_FLAG.global != Action::Unset {
                WARN_FLAG.global
            } else if WARN_VARIABLE.actions[i] != Action::Unset {
                WARN_VARIABLE.actions[i]
            } else if WARN_VARIABLE.global != Action::Unset {
                WARN_VARIABLE.global
            } else {
                WARN_DEFAULT.actions[i]
            };
        }
    }
}

pub fn init() {
    unsafe {
        WARN_DEFAULT = Data::default();
        WARN_VARIABLE = Data::default();
        WARN_FLAG = Data::default();

        WARN_DEFAULT.global = Action::Warn;
        WARN_DEFAULT.actions[Type::CircularDep as usize] = Action::Warn;
        WARN_DEFAULT.actions[Type::InvalidRef as usize] = Action::Warn;
        WARN_DEFAULT.actions[Type::InvalidVar as usize] = Action::Warn;
        WARN_DEFAULT.actions[Type::UndefinedVar as usize] = Action::Ignore;
    }
    refresh_warnings();
}

/// Parse a `--warn=...` value (or a `.WARNINGS` variable value) and update
/// either the flag-level data (`flocp == None`) or the variable-level data
/// (`flocp == Some(...)`).
pub fn decode_actions(value: &str, flocp: Option<*const Floc>) {
    let target_flag = flocp.is_none();
    let value = value.trim_start_matches(|c: char| c.is_ascii_whitespace() || c == ',');

    // Updating .WARNINGS with an empty value resets variable-level data.
    if !target_flag && value.is_empty() {
        unsafe {
            WARN_VARIABLE = Data::default();
        }
    }

    for token in value.split(|c: char| c.is_ascii_whitespace() || c == ',') {
        if token.is_empty() {
            continue;
        }
        if let Some(action) = Action::from_name(token) {
            unsafe {
                if target_flag {
                    WARN_FLAG.global = action;
                } else {
                    WARN_VARIABLE.global = action;
                }
            }
            continue;
        }
        let (name, action_part) = match token.split_once(':') {
            Some((n, a)) => (n, Some(a)),
            None => (token, None),
        };
        let ty = match Type::from_name(name) {
            Some(t) => t,
            None => {
                report_error(flocp, format!("unknown warning '{name}'"));
                continue;
            }
        };
        let action = match action_part {
            None => Action::Warn,
            Some(s) => match Action::from_name(s) {
                Some(a) => a,
                None => {
                    report_error(flocp, format!("unknown warning action '{s}'"));
                    continue;
                }
            },
        };
        unsafe {
            if target_flag {
                WARN_FLAG.actions[ty as usize] = action;
            } else {
                WARN_VARIABLE.actions[ty as usize] = action;
            }
        }
    }
    refresh_warnings();
}

fn report_error(flocp: Option<*const Floc>, msg: String) {
    let suffix = if flocp.is_none() { "" } else { ": ignored" };
    let formatted = format!("{msg}{suffix}\0");
    unsafe {
        match flocp {
            None => fatal(
                ::core::ptr::null(),
                formatted.len(),
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                formatted.as_ptr() as *const ::core::ffi::c_char,
            ),
            Some(fp) => error(
                fp,
                formatted.len(),
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                formatted.as_ptr() as *const ::core::ffi::c_char,
            ),
        }
    }
}

/// Render the current `--warn` flag value into the variable buffer at `fp`,
/// returning the new buffer end pointer. Returns `fp` unchanged if no
/// flag-level overrides are active.
///
/// # Safety
/// `fp` must point into a valid variable_buffer location; the underlying
/// buffer is grown as needed by `variable_buffer_output`.
pub unsafe fn encode_flag(mut fp: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let any_per_warning = WARN_FLAG.actions.iter().any(|a| *a != Action::Unset);
    if !any_per_warning && WARN_FLAG.global == Action::Unset {
        return fp;
    }

    fp = append(fp, " --warn");

    if !any_per_warning && WARN_FLAG.global == Action::Warn {
        return fp;
    }

    let mut sep = '=';
    if let Some(name) = WARN_FLAG.global.name() {
        fp = append_char(fp, sep);
        sep = ',';
        fp = append(fp, name);
    }

    if any_per_warning {
        for t in Type::ALL {
            let act = WARN_FLAG.actions[t as usize];
            if act == Action::Unset {
                continue;
            }
            fp = append_char(fp, sep);
            sep = ',';
            fp = append(fp, t.name());
            if act != Action::Warn {
                let action_name = act.name().unwrap();
                fp = append(fp, ":");
                fp = append(fp, action_name);
            }
        }
    }
    fp
}

unsafe fn append(fp: *mut ::core::ffi::c_char, s: &str) -> *mut ::core::ffi::c_char {
    variable_buffer_output(fp, s.as_ptr() as *const ::core::ffi::c_char, s.len())
}

unsafe fn append_char(fp: *mut ::core::ffi::c_char, c: char) -> *mut ::core::ffi::c_char {
    let byte = c as u8;
    variable_buffer_output(fp, &byte as *const u8 as *const ::core::ffi::c_char, 1)
}

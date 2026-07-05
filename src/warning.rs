use std::sync::Mutex;

use crate::expand::variable_buffer_output;
use crate::floc::Floc;
use crate::output::msg;

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
        Self::ALL
            .into_iter()
            .find(|t| t.name().eq_ignore_ascii_case(s))
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
        [Action::Ignore, Action::Warn, Action::Error]
            .into_iter()
            .find(|&a| a.name().unwrap().eq_ignore_ascii_case(s))
    }
}

#[derive(Default, Copy, Clone)]
pub struct Data {
    pub global: Action,
    pub actions: [Action; Type::COUNT],
}

#[derive(Default, Copy, Clone)]
struct State {
    /// Active per-warning action, indexed by `Type as usize`.
    warnings: [Action; Type::COUNT],
    default: Data,
    variable: Data,
    flag: Data,
}

const EMPTY_DATA: Data = Data {
    global: Action::Unset,
    actions: [Action::Unset; Type::COUNT],
};

static STATE: Mutex<State> = Mutex::new(State {
    warnings: [Action::Unset; Type::COUNT],
    default: EMPTY_DATA,
    variable: EMPTY_DATA,
    flag: EMPTY_DATA,
});

/// Active action for the given warning type.
pub fn action(t: Type) -> Action {
    STATE.lock().unwrap().warnings[t as usize]
}

/// Override the active action for `t`. Used by sites that temporarily
/// silence a warning around a known-noisy call (e.g. `~` expansion in
/// `read.rs`, `$SHELL` lookup in `job.rs`).
pub fn set_action(t: Type, a: Action) {
    STATE.lock().unwrap().warnings[t as usize] = a;
}

/// True if the warning is currently configured to emit (warn or error).
pub fn is_active(t: Type) -> bool {
    matches!(action(t), Action::Warn | Action::Error)
}

/// Resolve the active per-warning action by walking the precedence chain:
/// per-flag → flag-global → per-variable → variable-global → default.
fn refresh(state: &mut State) {
    for t in Type::ALL {
        let i = t as usize;
        state.warnings[i] = if state.flag.actions[i] != Action::Unset {
            state.flag.actions[i]
        } else if state.flag.global != Action::Unset {
            state.flag.global
        } else if state.variable.actions[i] != Action::Unset {
            state.variable.actions[i]
        } else if state.variable.global != Action::Unset {
            state.variable.global
        } else {
            state.default.actions[i]
        };
    }
}

pub fn init() {
    let mut s = STATE.lock().unwrap();
    *s = State::default();
    s.default.global = Action::Warn;
    s.default.actions[Type::CircularDep as usize] = Action::Warn;
    s.default.actions[Type::InvalidRef as usize] = Action::Warn;
    s.default.actions[Type::InvalidVar as usize] = Action::Warn;
    s.default.actions[Type::UndefinedVar as usize] = Action::Ignore;
    refresh(&mut s);
}

/// Parse a `--warn=...` value (or a `.WARNINGS` variable value) and update
/// either the flag-level data (`flocp == None`) or the variable-level data
/// (`flocp == Some(...)`).
pub fn decode_actions(ctx: &crate::execctx::ExecContext, value: &str, flocp: Option<&Floc>) {
    let target_flag = flocp.is_none();
    let value = value.trim_start_matches(|c: char| c.is_ascii_whitespace() || c == ',');

    enum ReportKind {
        Unknown,
        UnknownAction,
    }
    let mut errors: Vec<(ReportKind, String)> = Vec::new();

    {
        let mut s = STATE.lock().unwrap();

        // Updating .WARNINGS with an empty value resets variable-level data.
        if !target_flag && value.is_empty() {
            s.variable = Data::default();
        }

        for token in value.split(|c: char| c.is_ascii_whitespace() || c == ',') {
            if token.is_empty() {
                continue;
            }
            if let Some(action) = Action::from_name(token) {
                if target_flag {
                    s.flag.global = action;
                } else {
                    s.variable.global = action;
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
                    errors.push((ReportKind::Unknown, name.to_string()));
                    continue;
                }
            };
            let action = match action_part {
                None => Action::Warn,
                Some(s) => match Action::from_name(s) {
                    Some(a) => a,
                    None => {
                        errors.push((ReportKind::UnknownAction, s.to_string()));
                        continue;
                    }
                },
            };
            if target_flag {
                s.flag.actions[ty as usize] = action;
            } else {
                s.variable.actions[ty as usize] = action;
            }
        }
        refresh(&mut s);
    }

    // `report_error` may not return (`fatal` path), so we drop the lock
    // before invoking it.
    for (kind, name) in errors {
        let body = match kind {
            ReportKind::Unknown => format!("unknown warning '{name}'"),
            ReportKind::UnknownAction => format!("unknown warning action '{name}'"),
        };
        report_error(ctx, flocp, body);
    }
}

fn report_error(ctx: &crate::execctx::ExecContext, flocp: Option<&Floc>, message: String) {
    match flocp {
        None => msg::fatal(ctx, None, &message),
        Some(fp) => msg::error(ctx, Some(fp), &format!("{message}: ignored")),
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
    let flag = STATE.lock().unwrap().flag;
    let any_per_warning = flag.actions.iter().any(|a| *a != Action::Unset);
    if !any_per_warning && flag.global == Action::Unset {
        return fp;
    }

    fp = append(fp, " --warn");

    if !any_per_warning && flag.global == Action::Warn {
        return fp;
    }

    let mut sep = '=';
    if let Some(name) = flag.global.name() {
        fp = append_char(fp, sep);
        sep = ',';
        fp = append(fp, name);
    }

    if any_per_warning {
        for t in Type::ALL {
            let act = flag.actions[t as usize];
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_from_name_known_names() {
        assert_eq!(Action::from_name("warn"), Some(Action::Warn));
        assert_eq!(Action::from_name("ignore"), Some(Action::Ignore));
        assert_eq!(Action::from_name("error"), Some(Action::Error));
        // Case-insensitive match must also hit the correct variant.
        assert_eq!(Action::from_name("WARN"), Some(Action::Warn));
        // Unknown names must return None.
        assert_eq!(Action::from_name("xyzzy"), None);
    }

    #[test]
    fn type_from_name_known_names() {
        assert_eq!(Type::from_name("circular-dep"), Some(Type::CircularDep));
        assert_eq!(Type::from_name("invalid-ref"), Some(Type::InvalidRef));
        assert_eq!(Type::from_name("invalid-var"), Some(Type::InvalidVar));
        assert_eq!(Type::from_name("undefined-var"), Some(Type::UndefinedVar));
        // Unknown names must return None.
        assert_eq!(Type::from_name("xyzzy"), None);
    }
}

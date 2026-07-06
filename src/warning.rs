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

#[derive(Debug, Default, Copy, Clone)]
pub struct Data {
    pub global: Action,
    pub actions: [Action; Type::COUNT],
}

/// Warning configuration, held per-session on
/// [`crate::execctx::ExecContext::warning_state`] in place of the former
/// process-global `static STATE`. Pure POD (no pointers), so `Cell<State>`
/// gets a sound auto-derived `Clone`.
#[derive(Debug, Default, Copy, Clone)]
pub struct State {
    /// Active per-warning action, indexed by `Type as usize`.
    warnings: [Action; Type::COUNT],
    default: Data,
    variable: Data,
    flag: Data,
}

/// Active action for the given warning type.
pub fn action(ctx: &crate::execctx::ExecContext, t: Type) -> Action {
    ctx.warning_state.get().warnings[t as usize]
}

/// Override the active action for `t`. Used by sites that temporarily
/// silence a warning around a known-noisy call (e.g. `~` expansion in
/// `read.rs`, `$SHELL` lookup in `job.rs`).
pub fn set_action(ctx: &crate::execctx::ExecContext, t: Type, a: Action) {
    let mut s = ctx.warning_state.get();
    s.warnings[t as usize] = a;
    ctx.warning_state.set(s);
}

/// True if the warning is currently configured to emit (warn or error).
pub fn is_active(ctx: &crate::execctx::ExecContext, t: Type) -> bool {
    matches!(action(ctx, t), Action::Warn | Action::Error)
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

pub fn init(ctx: &crate::execctx::ExecContext) {
    let mut s = State::default();
    s.default.global = Action::Warn;
    s.default.actions[Type::CircularDep as usize] = Action::Warn;
    s.default.actions[Type::InvalidRef as usize] = Action::Warn;
    s.default.actions[Type::InvalidVar as usize] = Action::Warn;
    s.default.actions[Type::UndefinedVar as usize] = Action::Ignore;
    refresh(&mut s);
    ctx.warning_state.set(s);
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
        let mut s = ctx.warning_state.get();

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
        ctx.warning_state.set(s);
    }

    // `report_error` may not return (`fatal` path); the updated state above is
    // already written back before we risk that.
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
        None => msg::fatal(
            ctx,
            // SAFETY: the current output-sync target, resolved fresh here.
            unsafe { crate::output::output_context().as_mut() },
            None,
            &message,
        ),
        Some(fp) => msg::error(
            ctx,
            // SAFETY: the current output-sync target, resolved fresh here.
            unsafe { crate::output::output_context().as_mut() },
            Some(fp),
            &format!("{message}: ignored"),
        ),
    }
}

/// Render the current `--warn` flag value into the variable buffer at `fp`,
/// returning the new buffer end pointer. Returns `fp` unchanged if no
/// flag-level overrides are active.
///
/// # Safety
/// `fp` must point into a valid variable_buffer location; the underlying
/// buffer is grown as needed by `variable_buffer_output`.
pub unsafe fn encode_flag(
    ctx: &crate::execctx::ExecContext,
    mut fp: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let flag = ctx.warning_state.get().flag;
    let any_per_warning = flag.actions.iter().any(|a| *a != Action::Unset);
    if !any_per_warning && flag.global == Action::Unset {
        return fp;
    }

    fp = append(ctx, fp, " --warn");

    if !any_per_warning && flag.global == Action::Warn {
        return fp;
    }

    let mut sep = '=';
    if let Some(name) = flag.global.name() {
        fp = append_char(ctx, fp, sep);
        sep = ',';
        fp = append(ctx, fp, name);
    }

    if any_per_warning {
        for t in Type::ALL {
            let act = flag.actions[t as usize];
            if act == Action::Unset {
                continue;
            }
            fp = append_char(ctx, fp, sep);
            sep = ',';
            fp = append(ctx, fp, t.name());
            if act != Action::Warn {
                let action_name = act.name().unwrap();
                fp = append(ctx, fp, ":");
                fp = append(ctx, fp, action_name);
            }
        }
    }
    fp
}

unsafe fn append(
    ctx: &crate::execctx::ExecContext,
    fp: *mut ::core::ffi::c_char,
    s: &str,
) -> *mut ::core::ffi::c_char {
    variable_buffer_output(ctx, fp, s.as_ptr() as *const ::core::ffi::c_char, s.len())
}

unsafe fn append_char(
    ctx: &crate::execctx::ExecContext,
    fp: *mut ::core::ffi::c_char,
    c: char,
) -> *mut ::core::ffi::c_char {
    let byte = c as u8;
    variable_buffer_output(ctx, fp, &byte as *const u8 as *const ::core::ffi::c_char, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `warning_state` lives on `ExecContext` now (no longer a shared
    /// process-global), so each test just builds its own isolated context —
    /// no cross-test lock needed.
    #[test]
    fn decode_actions_flag_level_sets_active_action() {
        let ctx = crate::execctx::ExecContext::default();
        init(&ctx);
        decode_actions(&ctx, "undefined-var:error", None);
        assert_eq!(action(&ctx, Type::UndefinedVar), Action::Error);
    }

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

    #[test]
    fn set_action_and_is_active_cover_all_variants() {
        let ctx = crate::execctx::ExecContext::default();
        // Fresh context: every type starts Unset (not active).
        assert!(!is_active(&ctx, Type::CircularDep));
        set_action(&ctx, Type::CircularDep, Action::Ignore);
        assert_eq!(action(&ctx, Type::CircularDep), Action::Ignore);
        assert!(!is_active(&ctx, Type::CircularDep));
        set_action(&ctx, Type::CircularDep, Action::Warn);
        assert!(is_active(&ctx, Type::CircularDep));
        set_action(&ctx, Type::CircularDep, Action::Error);
        assert!(is_active(&ctx, Type::CircularDep));
        // Setting one type must not disturb another.
        assert!(!is_active(&ctx, Type::InvalidRef));
    }

    #[test]
    fn init_sets_the_documented_defaults() {
        let ctx = crate::execctx::ExecContext::default();
        init(&ctx);
        assert_eq!(action(&ctx, Type::CircularDep), Action::Warn);
        assert_eq!(action(&ctx, Type::InvalidRef), Action::Warn);
        assert_eq!(action(&ctx, Type::InvalidVar), Action::Warn);
        assert_eq!(action(&ctx, Type::UndefinedVar), Action::Ignore);
    }

    #[test]
    fn encode_flag_with_no_overrides_returns_fp_unchanged() {
        let _g = crate::expand::VARIABLE_BUFFER_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let ctx = crate::execctx::ExecContext::default();
        unsafe {
            let fp = crate::expand::initialize_variable_output(&ctx);
            assert_eq!(encode_flag(&ctx, fp), fp);
        }
    }

    #[test]
    fn encode_flag_renders_global_and_per_warning_overrides() {
        let _g = crate::expand::VARIABLE_BUFFER_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let ctx = crate::execctx::ExecContext::default();
        decode_actions(&ctx, "error", None);
        decode_actions(&ctx, "circular-dep:ignore", None);
        unsafe {
            let fp = crate::expand::initialize_variable_output(&ctx);
            let out = encode_flag(&ctx, fp);
            let rendered = ::core::ffi::CStr::from_ptr(fp).to_str().unwrap();
            assert_eq!(rendered, " --warn=error,circular-dep:ignore");
            assert_eq!(out, fp.add(rendered.len()));
        }
    }

    /// With a location, `report_error` reports via `msg::error` (returns,
    /// rather than the `None`/`msg::fatal` arm) and appends `": ignored"` —
    /// drives it through a real temp-fd sync target so the emitted bytes can
    /// be read back and asserted on, instead of only exercising it
    /// indirectly through `decode_actions`'s unknown-name warnings.
    #[test]
    fn report_error_with_location_writes_the_ignored_message() {
        crate::make_main::install_default_exec_context_for_test();
        crate::make_main::install_default_options_for_test();
        let ctx = crate::execctx::ExecContext::default();

        let path = std::env::temp_dir().join(format!(
            "report-error-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let file = std::fs::File::options()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)
            .expect("open temp file");
        use std::os::unix::io::IntoRawFd;
        let fd = file.into_raw_fd();
        let mut out = crate::output::output {
            out: fd,
            err: fd,
            syncout: [0; 1],
            c2rust_padding: [0; 3],
        };
        out.set_syncout(1);
        crate::output::set_output_context(&mut out as *mut _);

        let floc = Floc {
            filenm: c"Makefile".as_ptr(),
            lineno: 3,
            offset: 0,
        };
        report_error(&ctx, Some(&floc), "bogus warning name".to_string());

        crate::output::set_output_context(::core::ptr::null_mut());
        use std::os::unix::io::FromRawFd;
        drop(unsafe { std::fs::File::from_raw_fd(fd) });

        let contents = std::fs::read_to_string(&path).expect("read temp file");
        assert!(
            contents.contains("Makefile:3: bogus warning name: ignored"),
            "unexpected output: {contents:?}"
        );
        let _ = std::fs::remove_file(&path);
    }
}

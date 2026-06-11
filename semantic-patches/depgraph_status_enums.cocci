// Replace the c2rust integer constants for File status with real enum
// variants (UpdateStatus / CommandState in src/file.rs), stripping the
// integer-alias casts c2rust emitted alongside them.

@us_failed_cast@
@@
- us_failed as update_status
+ UpdateStatus::Failed

@us_question_cast@
@@
- us_question as update_status
+ UpdateStatus::Question

@us_none_cast@
@@
- us_none as update_status
+ UpdateStatus::None

@us_success_cast@
@@
- us_success as update_status
+ UpdateStatus::Success

@us_failed@
@@
- us_failed
+ UpdateStatus::Failed

@us_question@
@@
- us_question
+ UpdateStatus::Question

@us_none@
@@
- us_none
+ UpdateStatus::None

@us_success@
@@
- us_success
+ UpdateStatus::Success

@cs_finished@
@@
- cs_finished
+ CommandState::Finished

@cs_running@
@@
- cs_running
+ CommandState::Running

@cs_deps_running@
@@
- cs_deps_running
+ CommandState::DepsRunning

@cs_not_started@
@@
- cs_not_started
+ CommandState::NotStarted

@set_update_status@
expression F, V;
@@
- F.set_update_status(V)
+ F.update_status = V

@get_update_status@
expression F;
@@
- F.update_status()
+ F.update_status

@set_command_state_method@
expression F, V;
@@
- F.set_command_state(V)
+ F.command_state = V

@get_command_state@
expression F;
@@
- F.command_state()
+ F.command_state

# Night Light Toggle

English | [Русский](README_ru.md)

A lightweight Windows 11 utility written in Rust that reapplies Night light.

On launch, it disables Night light, waits one second, enables it, and exits.
The final state is always enabled, regardless of the initial state.
The schedule and intensity remain unchanged. Successful runs show no windows;
errors appear in a MessageBox.

Run `night-light-toggle.exe` from this folder. Administrator privileges are not
required. The utility does not stay in the background or register itself for
automatic startup.

## Automatic startup

The local setup uses a Task Scheduler task named `Night Light Refresh`:

- Runs 10 seconds after the current user signs in to Windows.
- Runs 10 seconds after a wake event: System / Microsoft-Windows-Power-Troubleshooter / Event ID 1 (sleep or hibernation).

The task runs as the current user without elevated privileges and requires an
existing user session. It does not wake the computer. The local task configuration
is saved in `scheduled-task.xml`, which is excluded from this repository.
The executable must remain at the path configured in the task.

To remove automatic startup:

```powershell
Unregister-ScheduledTask -TaskName 'Night Light Refresh' -Confirm:$false
```

## Build

```powershell
cargo build --release --locked
Copy-Item target\release\night-light-toggle.exe .\night-light-toggle.exe
```

Requires Rust stable with the MSVC toolchain, C++ Build Tools, and the Windows SDK.

## Validation

```powershell
cargo clippy --all-targets --locked -- -D warnings
cargo test --release --locked --test windows_smoke -- --ignored --nocapture
```

The integration test checks the disabled phase, the final enabled state, and
process termination from both initial states. It temporarily changes Night light,
then restores the original state. The schedule and intensity must remain unchanged.
The test does not measure the actual colors displayed by the monitors.

Night light access uses `win-nightlight-lib` 0.2.0. It relies on the undocumented
CloudStore format, so compatibility with future Windows versions is not guaranteed.
The reported state represents the Windows setting, not the actual color output of
each monitor.

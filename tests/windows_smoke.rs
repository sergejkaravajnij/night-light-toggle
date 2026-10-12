use std::process::{Child, Command};
use std::thread::sleep;
use std::time::{Duration, Instant};
use win_nightlight_lib::Nightlight;

struct RestoreState(bool);
impl Drop for RestoreState {
    fn drop(&mut self) {
        Nightlight::new()
            .set_active(self.0)
            .expect("restore original state");
    }
}
struct StopChild(Child);
impl Drop for StopChild {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}
#[test]
#[ignore = "temporarily changes this user's Night light"]
fn disables_then_enables_and_exits_from_either_initial_state() {
    let nightlight = Nightlight::new();
    let _restore = RestoreState(nightlight.read_state().unwrap().is_active());
    let settings = nightlight.read_settings().unwrap().to_bytes().unwrap();
    for before in [true, false] {
        nightlight.set_active(before).unwrap();
        sleep(Duration::from_secs(1));
        let mut child = StopChild(
            Command::new(env!("CARGO_BIN_EXE_night-light-toggle"))
                .spawn()
                .unwrap(),
        );
        let deadline = Instant::now() + Duration::from_secs(10);
        let mut observed_off = false;
        loop {
            if let Some(status) = child.0.try_wait().unwrap() {
                assert!(status.success(), "application failed: {status}");
                break;
            }
            observed_off |= !nightlight.read_state().unwrap().is_active();
            assert!(Instant::now() < deadline, "application did not exit");
            sleep(Duration::from_millis(20));
        }
        assert!(observed_off, "off phase was not observed");
        assert!(nightlight.read_state().unwrap().is_active());
        assert_eq!(
            nightlight.read_settings().unwrap().to_bytes().unwrap(),
            settings
        );
    }
}

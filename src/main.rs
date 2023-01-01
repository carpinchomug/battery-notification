use notify_rust::{Hint, Notification, Urgency};
use tokio_stream::StreamExt;
use upower_dbus::{BatteryState, UPowerProxy, WarningLevel};
use zbus::Connection;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let connection = Connection::system().await?;

    let upower = UPowerProxy::new(&connection).await?;

    let device = upower.get_display_device().await?;

    let warn_low = Notification::new()
        .summary("Battery level")
        .body("Low")
        .hint(Hint::Urgency(Urgency::Normal))
        .finalize();

    let warn_critical = Notification::new()
        .summary("Battery level")
        .body("Critically low")
        .hint(Hint::Urgency(Urgency::Critical))
        .finalize();

    let notify_full = Notification::new()
        .summary("Battery level")
        .body("Fully charged")
        .hint(Hint::Urgency(Urgency::Normal))
        .finalize();

    let mut stream_warning_level = device.receive_warning_level_changed().await;
    let mut stream_state = device.receive_state_changed().await;

    loop {
        tokio::select! {
            Some(warning_level) = stream_warning_level.next() => {
                if let BatteryState::Discharging = device.state().await? {
                    match warning_level.get().await? {
                        WarningLevel::Low => {
                            warn_low.show()?;
                        },
                        WarningLevel::Critical => {
                            warn_critical.show()?;
                        },
                        _ => (),
                    }
                }
            }
            Some(state) = stream_state.next() => {
                if let BatteryState::FullyCharged = state.get().await? {
                    notify_full.show()?;
                }
            }
            else => { break }
        }
    }

    Ok(())
}

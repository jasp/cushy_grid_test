use chrono::{DateTime, Utc};
use cushy::{value::{Dynamic, Source}, widget::MakeWidget, widgets::{grid::{GridDimension, GridWidgets}, Grid}, Open, PendingApp, TokioRuntime};

struct Telegram {
    timestamp: DateTime<Utc>,
    band_freq: f64,
    rssi: f64,
}

fn main() {
    let rt = TokioRuntime::default();
    let telegrams = Dynamic::new(vec![]);

    let view = telegrams_view(telegrams.clone());

    rt.spawn(async move {
        for _ in 0..1000 {
            telegrams.lock().push(Telegram {
                timestamp: Utc::now(),
                band_freq: 868900000f64 + rand::random::<f64>() * 100000f64,
                rssi: -10f64 - rand::random::<f64>() * 100f64,
            });
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    });

    let app = PendingApp::new(rt);
    let win = view
    .into_window();

    win
    .run_in(app)
    .unwrap();
}

fn telegrams_view(telegrams: Dynamic<Vec<Telegram>>) -> impl MakeWidget {
    let framelist = telegrams.map_each(move |telegrams| {
        telegrams
            .iter()
            .fold(GridWidgets::from(("Timestamp".align_left(), "RSSI".align_left(), "Band frequency".align_left())),
            |res, telegram| {
                res.and((
                    telegram.timestamp.to_rfc2822().align_left(),
                    format!("{} dBm", telegram.rssi).align_left(),
                    format!("{} kHz", telegram.band_freq / 1000f64).align_left()
                ))
            })
    });
    Grid::from_rows(framelist)
    .dimensions([
        GridDimension::FitContent,
        GridDimension::FitContent,
        GridDimension::FitContent,
    ])
    .align_left()
    .vertical_scroll()
    .expand()
}

mod data;
mod winsys;

use data::{list_videos, Data};

use std::borrow::Cow;

struct App {
    data: Data,
    sel_channel: usize,
    sel_video: usize,
    refresh: bool,
}

impl App {
    fn new() -> App {
        let mut data = list_videos("/mnt/freenas_misc/vidl".into());
        data.sort_videos();

        let mut channels = vec![];
        for (chan_title, chan_info) in &data.channels {
            channels.push((chan_title.clone(), chan_info.videos.len()));
        }

        channels.sort_by(|a, b| a.partial_cmp(&b).unwrap());

        App {
            data,
            sel_channel: 1,
            sel_video: 0,
            refresh: false,
        }
    }
}

pub fn main() -> anyhow::Result<()> {
    let mut app = App::new();

    let mut channel_items: Vec<String> = app.data.channels.keys().cloned().collect();
    channel_items.sort_unstable_by_key(|x| x.to_ascii_lowercase());

    let mut video_items: Vec<imgui::ImString> = vec![];

    crate::winsys::run(move |ui| {
        ui.show_metrics_window(&mut true);

        ui.window("Main")
            .position([0.0, 0.0], imgui::Condition::Always)
            .size(ui.io().display_size, imgui::Condition::Always)
            .build(|| {
                if ui.button("Reload") {
                    app.data = list_videos("/mnt/freenas_misc/vidl".into())
                }

                let chan_list_changed = imgui::ListBox::new("##Channel List")
                    .size([
                        ui.content_region_avail()[0] / 2.0,
                        ui.content_region_avail()[1] - 30.0,
                    ])
                    .build_simple(ui, &mut app.sel_channel, &channel_items, &|x| {
                        Cow::Owned(format!("{}", x))
                    });

                if chan_list_changed || app.refresh {
                    let sel_channel_name = &channel_items[app.sel_channel];
                    let x = &app.data.channels[&sel_channel_name.to_string()];
                    video_items = x
                        .videos
                        .iter()
                        .map(|v| imgui::ImString::new(&v.title))
                        .collect();
                    app.refresh = false;
                }
                ui.same_line();
                imgui::ListBox::new("##Videos")
                    .size([
                        ui.content_region_avail()[0],
                        ui.content_region_avail()[1] - 30.0,
                    ])
                    .build_simple(ui, &mut app.sel_video, &video_items, &|x| {
                        Cow::Owned(format!("{}", x))
                    });
                if ui.button_with_size("Mark watched", [100.0, 28.0]) {
                    let chan_name = channel_items[app.sel_channel].to_string();
                    let video = &app.data.channels[&chan_name].videos[app.sel_video];
                    let p = std::path::Path::new(&video.path);
                    let dest = p
                        .parent()
                        .unwrap()
                        .join("watched")
                        .join(p.file_name().unwrap());
                    std::fs::rename(p, dest).unwrap();
                    app.data
                        .channels
                        .get_mut(&chan_name)
                        .unwrap()
                        .videos
                        .remove(app.sel_video);
                    app.refresh = true;
                }

                ui.same_line_with_spacing(-1.0, 200.0);

                if ui.button_with_size("Play", [-100.0, 28.0]) {
                    let chan_name = channel_items[app.sel_channel].to_string();
                    let video = &app.data.channels[&chan_name].videos[app.sel_video];
                    std::process::Command::new("xdg-open")
                        .arg(&video.path)
                        .spawn()
                        .unwrap();
                }
            });
    });

    Ok(())
}

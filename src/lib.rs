mod data;
mod winsys;

use std::path::PathBuf;

use data::{list_videos, Data};

struct App {
    root_path: PathBuf,
    data: Data,
    sel_channel: Option<String>,
    sel_video: Option<String>,
    search: String,
}

impl App {
    fn new() -> App {
        let root_path = "/mnt/hypercube/misc/vidl".into();
        let mut data = list_videos(&root_path);
        data.sort_videos();

        App {
            root_path,
            data,
            sel_channel: None,
            sel_video: None,
            search: "".into(),
        }
    }

    fn refresh(&mut self) {
        self.data = list_videos(&self.root_path);
        self.data.sort_videos();
    }
}

pub fn main() -> anyhow::Result<()> {
    let mut app = App::new();

    crate::winsys::run(move |ui| {
        ui.window("Main Window")
            .position([0.0, 0.0], imgui::Condition::Always)
            .size(ui.io().display_size, imgui::Condition::Always)
            .build(|| {
                if ui.button("Reload") {
                    app.refresh();
                }

                ui.same_line();

                ui.input_text("##Search", &mut app.search).build();

                ui.same_line();

                if ui.button("clear") {
                    app.search.clear();
                }

                imgui::ListBox::new("##Channel List")
                    .size([
                        ui.content_region_avail()[0] / 2.0,
                        ui.content_region_avail()[1] - 30.0,
                    ])
                    .build(ui, || {
                        for (i, chan) in app.data.channel_list(&app.search).iter().enumerate() {
                            let selected = match app.sel_channel {
                                Some(ref x) => x == chan,
                                None => false,
                            };
                            if ui
                                .selectable_config(format!("{}##channel {}", &chan, i))
                                .selected(selected)
                                .build()
                            {
                                app.sel_channel = Some(chan.to_string());
                            }
                        }
                    });

                ui.same_line();
                imgui::ListBox::new("##Videos")
                    .size([
                        ui.content_region_avail()[0],
                        ui.content_region_avail()[1] - 30.0,
                    ])
                    .build(ui, || {
                        if let Some(sc) = &app.sel_channel {
                            let video_items: Vec<&str> = app.data.list_videos(&sc, &app.search);
                            for (i, video) in video_items.iter().enumerate() {
                                let selected = match app.sel_video {
                                    Some(ref x) => x == video,
                                    None => false,
                                };
                                if ui
                                    .selectable_config(format!("{}##video {}", &video, i))
                                    .selected(selected)
                                    .build()
                                {
                                    app.sel_video = Some(video.to_string());
                                }
                            }
                        }
                    });

                if ui.button_with_size("Mark watched", [100.0, 28.0]) {
                    if let Some(sc) = &app.sel_channel {
                        if let Some(sv) = &app.sel_video {
                            if let Some(video) = app.data.get_video(&sc, &sv) {
                                let p = std::path::Path::new(&video.path);
                                let dest = p
                                    .parent()
                                    .unwrap()
                                    .join("watched")
                                    .join(p.file_name().unwrap());
                                std::fs::rename(p, dest).unwrap();
                                app.sel_video = None;
                                app.refresh();
                            }
                        }
                    }
                }

                ui.same_line_with_spacing(-1.0, 200.0);

                if ui.button_with_size("Play", [-100.0, 28.0]) {
                    if let Some(sc) = &app.sel_channel {
                        if let Some(sv) = &app.sel_video {
                            if let Some(video) = app.data.get_video(&sc, &sv) {
                                std::process::Command::new("xdg-open")
                                    .arg(&video.path)
                                    .spawn()
                                    .unwrap();
                            }
                        }
                    }
                }
            });
    });

    Ok(())
}

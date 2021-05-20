mod winsys;
mod data;

use data::{Data, list_videos};

use std::{borrow::Cow, path::PathBuf};

struct App {
    data: Data,
    sel_channel: usize,
    sel_video: usize,
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

        App { data, sel_channel: 1, sel_video: 0 }
    }
}


pub fn main() -> anyhow::Result<()> {
    let mut app = App::new();

    use imgui::im_str;
    let channel_items: Vec<imgui::ImString> = app.data.channels.keys().map(|x| imgui::ImString::new(x)).collect();
    let mut video_items: Vec<imgui::ImString> = vec![imgui::ImString::new("test")];

    crate::winsys::run(move |ui| {
        let chan_list_changed = imgui::ListBox::new(im_str!("##Channel List")).build_simple(
            ui,
            &mut app.sel_channel,
            &channel_items,
            &|x| Cow::Owned(imgui::ImString::new(format!("{}", x))),
        );

        if chan_list_changed {
            let sel_channel_name = &channel_items[app.sel_channel];
            let x = &app.data.channels[&sel_channel_name.to_string()];
            video_items = x.videos.iter().map(|v| imgui::ImString::new(&v.title)).collect();
        }
        imgui::ListBox::new(im_str!("##Videos")).build_simple(
            ui,
            &mut app.sel_video,
            &video_items,
            &|x| Cow::Owned(imgui::ImString::new(format!("{}", x))),
        );
    });
    Ok(())
}

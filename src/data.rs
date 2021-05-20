use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Video {
    pub title: String,
    pub path: PathBuf,
}
#[derive(Debug)]
pub struct Channel {
    pub videos: Vec<Video>,
}

#[derive(Debug)]
pub struct Data {
    pub channels: HashMap<String, Channel>,
}

impl Data {
    pub fn sort_videos(&mut self) {
        for c in self.channels.values_mut() {
            c.videos
                .sort_by(|a, b| a.path.partial_cmp(&b.path).unwrap());
        }
    }
}


pub fn list_videos(path: std::path::PathBuf) -> Data {
    let mut ret = Data {
        channels: HashMap::new(),
    };

    let mut files = vec![];
    for f in std::fs::read_dir(path).unwrap() {
        files.push(f.unwrap());
    }

    files.sort_by(|a, b| a.path().partial_cmp(&b.path()).unwrap());

    for info in files {
        if info.file_type().unwrap().is_file() {
            let raw_filename = info.file_name();
            let filename = raw_filename.to_str().unwrap();
            let (chan, title) = match filename.find("__") {
                Some(idx) => filename.split_at(idx),
                None => continue,
            };
            let title = title.split_at(3).1;
            ret.channels
                .entry(chan.into())
                .or_insert(Channel { videos: vec![] })
                .videos
                .push(Video {
                    title: title.into(),
                    path: info.path(),
                });
        }
    }

    ret
}
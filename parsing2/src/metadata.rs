use std::collections::HashSet;

lazy_static! {
    pub static ref QUALITY: HashSet<&'static str> = hashset!{
        "2160p",
        "1080p",
        "720p",
        "480p",
        "360p",
        "240p",
    };
    pub static ref VIDEO_FORMAT: HashSet<&'static str> = hashset!{
        "xvid",
        "divx",
        "h264",
        "x264",
        "h265",
        "x265",
        "10bit",
    };
    pub static ref AUDIO_FORMAT: HashSet<&'static str> = hashset!{
        "ac3",
        "aac",
        "aac2",
        "dd5",
        "dd2",
    };
    pub static ref ALL: HashSet<&'static str> = {
        QUALITY
            .iter()
            .chain(VIDEO_FORMAT.iter())
            .chain(AUDIO_FORMAT.iter())
            .cloned()
            .collect()
    };
    pub static ref VIDEO_EXT: HashSet<&'static str> = hashset!{
        "mkv",
        "mp4",
        "avi",
        "m4v",
        "webm",
        "flv",
        "vob",
        "mov",
        "wmv",
        "ogv",
        "ogg",
    };
    pub static ref SUBTITLE_EXT: HashSet<&'static str> = hashset!{
        "srt",
        "sub",
        "idx",
        "usf",
        "smi",
    };
}

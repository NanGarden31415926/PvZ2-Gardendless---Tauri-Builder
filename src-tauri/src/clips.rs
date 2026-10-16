use std::collections::HashMap;

const AUDIO_JSON: &str = include_str!("../assets/audio.json");
 
static AUDIO_FILES: &[(&str, &[u8])] = &[
    // ("blablabla.mp3", include_bytes!("../assets/audio.mp3")),
];

pub fn build_clips() -> HashMap<String, &'static [u8]> {
    let map: HashMap<String, String> = serde_json::from_str(AUDIO_JSON)
        .expect("assets/audio.json 解析失败");

    let files: HashMap<&str, &'static [u8]> = AUDIO_FILES.iter().copied().collect();

    let mut result = HashMap::new();
    for (name, filename) in &map {
        if let Some(bytes) = files.get(filename.as_str()) {
            result.insert(name.clone(), *bytes);
        } else {
            eprintln!("[clips] {} 对应的 {} 未注册", name, filename);
        }
    }
    result
}
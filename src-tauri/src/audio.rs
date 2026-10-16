use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink, Source};
use std::collections::HashMap;
use std::io::Cursor;
use std::sync::{Arc, Mutex, OnceLock};

use crate::clips;
static STREAM: OnceLock<OutputStream> = OnceLock::new();

struct AudioObject {
    sink: Sink,
}

pub struct AudioEngine {
    objects: Arc<Mutex<HashMap<u32, AudioObject>>>,
    next_id: Arc<Mutex<u32>>,
    clips: HashMap<String, &'static [u8]>,
}

impl AudioEngine {
    pub fn new() -> Result<Self, String> {
        let stream = OutputStreamBuilder::open_default_stream()
            .map_err(|e| e.to_string())?;
        STREAM.set(stream).map_err(|_| "already initialized")?;

        Ok(Self {
            objects: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
            clips: clips::build_clips(),
        })
    }

    /// 播放一个音效，speed 控制变速变调
    pub fn play(&self, name: &str, speed: f32) -> Result<u32, String> {
        let bytes = self.clips.get(name)
            .ok_or(format!("clip not found: {}", name))?;

        let stream = STREAM.get().ok_or("audio not initialized")?;
        let sink = Sink::connect_new(&stream.mixer());

        // 不复制字节，Cursor 借用静态数据
        let cursor = Cursor::new(*bytes);
        let source = Decoder::try_from(cursor).map_err(|e| e.to_string())?;
        let source = source.speed(speed);

        sink.append(source);

        let id = {
            let mut n = self.next_id.lock().unwrap();
            let id = *n;
            *n += 1;
            id
        };

        self.objects.lock().unwrap().insert(id, AudioObject { sink });
        Ok(id)
    }

    /// 实时变速变调
    pub fn set_speed(&self, id: u32, speed: f32) -> Result<(), String> {
        let objs = self.objects.lock().unwrap();
        let obj = objs.get(&id).ok_or("object not found")?;
        obj.sink.set_speed(speed);
        Ok(())
    }

    /// 音量 0.0 ~ 1.0
    pub fn set_volume(&self, id: u32, v: f32) -> Result<(), String> {
        let objs = self.objects.lock().unwrap();
        let obj = objs.get(&id).ok_or("object not found")?;
        obj.sink.set_volume(v.clamp(0.0, 1.0));
        Ok(())
    }

    /// 暂停 / 恢复
    pub fn set_playing(&self, id: u32, p: bool) -> Result<(), String> {
        let objs = self.objects.lock().unwrap();
        let obj = objs.get(&id).ok_or("object not found")?;
        if p { obj.sink.play(); } else { obj.sink.pause(); }
        Ok(())
    }

    /// 停止并销毁
    pub fn destroy(&self, id: u32) -> Result<(), String> {
        if let Some(obj) = self.objects.lock().unwrap().remove(&id) {
            obj.sink.stop();
        }
        Ok(())
    }

    /// 列出所有已注册的音效名
    pub fn list_clips(&self) -> Vec<String> {
        self.clips.keys().cloned().collect()
    }
}

pub struct AudioState(pub Mutex<AudioEngine>);
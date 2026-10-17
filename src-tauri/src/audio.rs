use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink, Source};
use std::collections::HashMap;
use std::io::Cursor;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;
use crate::clips;

static STREAM: OnceLock<OutputStream> = OnceLock::new();

struct AudioObject {
    sink: Sink,
    total_duration: Option<Duration>,
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

    pub fn play(&self, name: &str, speed: f32, volume: f32) -> Result<u32, String> {
        let bytes = self.clips.get(name)
            .ok_or(format!("clip not found: {}", name))?;

        let stream = STREAM.get().ok_or("audio not initialized")?;
        let sink = Sink::connect_new(&stream.mixer());

        let cursor = Cursor::new(*bytes);
        let source = Decoder::try_from(cursor).map_err(|e| e.to_string())?;

        let total_duration = match source.total_duration() {
            Some(d) => Some(d),
            None => mp3_duration::from_read(&mut Cursor::new(*bytes)).ok(),
        };

        let source = source.speed(speed);
        sink.set_volume(volume.clamp(0.0, 1.0));
        sink.append(source);

        let id = {
            let mut n = self.next_id.lock().unwrap();
            let id = *n;
            *n += 1;
            id
        };

        self.objects.lock().unwrap().insert(id, AudioObject { sink, total_duration });
        Ok(id)
    }
 
    pub fn cleanup_finished(&self) -> usize {
        let mut objs = self.objects.lock().unwrap();
        let finished: Vec<u32> = objs.iter()
            .filter(|(_, obj)| obj.sink.empty())
            .map(|(id, _)| *id)
            .collect();
        let count = finished.len();
        for id in finished {
            if let Some(obj) = objs.remove(&id) {
                obj.sink.stop();
            }
        }
        count
    }

    pub fn get_duration(&self, id: u32) -> Result<Option<f64>, String> {
        let objs = self.objects.lock().unwrap();
        let obj = objs.get(&id).ok_or("object not found")?;
        Ok(obj.total_duration.map(|d| d.as_secs_f64()))
    }

    pub fn set_speed(&self, id: u32, speed: f32) -> Result<(), String> {
        let objs = self.objects.lock().unwrap();
        let obj = objs.get(&id).ok_or("object not found")?;
        obj.sink.set_speed(speed);
        Ok(())
    }

    pub fn set_volume(&self, id: u32, v: f32) -> Result<(), String> {
        let objs = self.objects.lock().unwrap();
        let obj = objs.get(&id).ok_or("object not found")?;
        obj.sink.set_volume(v.clamp(0.0, 1.0));
        Ok(())
    }

    pub fn set_playing(&self, id: u32, p: bool) -> Result<(), String> {
        let objs = self.objects.lock().unwrap();
        let obj = objs.get(&id).ok_or("object not found")?;
        if p { obj.sink.play(); } else { obj.sink.pause(); }
        Ok(())
    }

    pub fn destroy(&self, id: u32) -> Result<(), String> {
        if let Some(obj) = self.objects.lock().unwrap().remove(&id) {
            obj.sink.stop();
        }
        Ok(())
    }

    pub fn list_clips(&self) -> Vec<String> {
        self.clips.keys().cloned().collect()
    }
}

pub struct AudioState(pub Mutex<AudioEngine>);
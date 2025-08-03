use rodio::{Decoder, OutputStream, Sink, Source};
use std::io::Cursor;
use std::time::Duration;
use rodio::source::{SineWave, TakeDuration};

pub struct AudioSystem {
    _stream: OutputStream,
    music_sink: Sink,
    sfx_sink: Sink,
    footstep_timer: f64,
    last_footstep: f64,
}

impl AudioSystem {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap_or_else(|_| {
            // Fallback if audio fails
            println!("Warning: Could not initialize audio");
            OutputStream::try_default().unwrap()
        });
        
        let music_sink = Sink::try_new(&stream_handle).unwrap();
        let sfx_sink = Sink::try_new(&stream_handle).unwrap();
        
        Self {
            _stream,
            music_sink,
            sfx_sink,
            footstep_timer: 0.0,
            last_footstep: 0.0,
        }
    }

    pub fn play_menu_music(&self) {
        self.music_sink.stop();
        
        // Generate a simple menu tune
        let source = self.generate_menu_music();
        self.music_sink.append(source);
        self.music_sink.set_volume(0.3);
    }

    pub fn play_game_music(&self) {
        self.music_sink.stop();
        
        // Generate ambient game music
        let source = self.generate_game_music();
        self.music_sink.append(source);
        self.music_sink.set_volume(0.2);
    }

    pub fn play_footstep(&mut self) {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        
        // Limit footstep frequency
        if current_time - self.last_footstep > 0.3 {
            let source = self.generate_footstep_sound();
            self.sfx_sink.append(source);
            self.last_footstep = current_time;
        }
    }

    pub fn play_success_sound(&self) {
        let source = self.generate_success_sound();
        self.sfx_sink.append(source);
    }

    pub fn play_death_sound(&self) {
        let source = self.generate_death_sound();
        self.sfx_sink.append(source);
    }

    pub fn play_damage_sound(&self) {
        let source = self.generate_damage_sound();
        self.sfx_sink.append(source);
    }

    // Generate procedural audio
    fn generate_menu_music(&self) -> SineWave {
        SineWave::new(440.0)
    }

    fn generate_game_music(&self) -> SineWave {
        SineWave::new(220.0)
    }

    fn generate_footstep_sound(&self) -> TakeDuration<SineWave> {
        SineWave::new(800.0).take_duration(Duration::from_millis(100))
    }

    fn generate_success_sound(&self) -> TakeDuration<SineWave> {
        SineWave::new(880.0).take_duration(Duration::from_secs(1))
    }

    fn generate_death_sound(&self) -> TakeDuration<SineWave> {
        SineWave::new(110.0).take_duration(Duration::from_secs(2))
    }

    fn generate_damage_sound(&self) -> TakeDuration<SineWave> {
        SineWave::new(1000.0).take_duration(Duration::from_millis(300))
    }

    pub fn stop_all(&self) {
        self.music_sink.stop();
        self.sfx_sink.stop();
    }

    pub fn set_music_volume(&self, volume: f32) {
        self.music_sink.set_volume(volume);
    }

    pub fn set_sfx_volume(&self, volume: f32) {
        self.sfx_sink.set_volume(volume);
    }
}
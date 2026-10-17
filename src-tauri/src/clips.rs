use std::collections::HashMap;

static AUDIO_FILES: &[(&str, &[u8])] = &[
    ("Music_Splash_Intro", include_bytes!("../Music/SplashIntro.mp3")),
    ("Game_2x_On", include_bytes!("../Sounds/Game/UI/2x_On.mp3")),
    ("Game_2x_Off", include_bytes!("../Sounds/Game/UI/2x_Off.mp3")),
    ("Game_SeedChooserOn", include_bytes!("../Sounds/Game/UI/SeedChooserOn.mp3")),
    ("Game_SeedChooserOff", include_bytes!("../Sounds/Game/UI/SeedChooserOff.mp3")),
    ("Game_ShovelActivate", include_bytes!("../Sounds/Game/UI/ShovelActivate.mp3")),
    ("Game_Pause", include_bytes!("../Sounds/Game/UI/Pause.mp3")),
    ("Game_Awooga", include_bytes!("../Sounds/Game/UI/Awooga.mp3")),
    ("Game_SandStormWarn", include_bytes!("../Sounds/Game/UI/SandStormWarn.mp3")),
    ("Game_ReadySet", include_bytes!("../Sounds/Game/UI/ReadySet.mp3")),
    ("Game_HugeWave", include_bytes!("../Sounds/Game/UI/HugeWave.mp3")),
    ("Game_FinalWave", include_bytes!("../Sounds/Game/UI/FinalWave.mp3")),
    ("Game_ZombieAlert", include_bytes!("../Sounds/Game/UI/ZombieAlert.mp3")),
];

pub fn build_clips() -> HashMap<String, &'static [u8]> {
    AUDIO_FILES
        .iter()
        .map(|(name, bytes)| (name.to_string(), *bytes))
        .collect()
}
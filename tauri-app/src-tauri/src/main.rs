use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tauri::{generate_handler, State};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct EmotionFrame {
    frame_type: String,
    path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CharacterData {
    name: String,
    emotions: Vec<Emotion>,
}

#[derive(Debug, Default, Clone)]
struct AudioState {
    volume_percent: Arc<Mutex<f32>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Emotion {
    name: String,
    frames: HashMap<String, EmotionFrame>,
}

#[tauri::command]
fn save_character(data: CharacterData, output_path: String) -> Result<(), String> {
    let mut output_path = output_path;
    if !output_path.ends_with(".basedchar") {
        output_path.push_str(".basedchar");
    }

    let path = std::path::Path::new(&output_path);
    let file = std::fs::File::create(path).map_err(|e| e.to_string())?;

    let mut zip = zip::ZipWriter::new(file);
    zip.start_file::<_, ()>("character.toml", zip::write::FileOptions::default())
        .map_err(|e| e.to_string())?;

    let mut updated_data = data.clone();
    for emotion in &mut updated_data.emotions {
        for frame in emotion.frames.values_mut() {
            let file_name = std::path::Path::new(&frame.path)
                .file_name()
                .ok_or_else(|| format!("Invalid file path: {}", frame.path))?
                .to_string_lossy()
                .to_string();
            frame.path = file_name;
        }
    }

    let toml_data = toml::to_string(&updated_data).map_err(|e| e.to_string())?;
    zip.write_all(toml_data.as_bytes()).map_err(|e| e.to_string())?;

    for emotion in &data.emotions {
        for frame in emotion.frames.values() {
            let img_path = std::path::Path::new(&frame.path);
            if img_path.exists() {
                let file_name = img_path.file_name()
                    .ok_or_else(|| format!("Invalid file path: {}", frame.path))?
                    .to_string_lossy()
                    .to_string();

                zip.start_file::<_, ()>(file_name, zip::write::FileOptions::default())
                    .map_err(|e| e.to_string())?;

                let mut img_file = std::fs::File::open(img_path).map_err(|e| e.to_string())?;
                std::io::copy(&mut img_file, &mut zip).map_err(|e| e.to_string())?;
            } else {
                return Err(format!("Image file not found: {}", frame.path));
            }
        }
    }

    zip.finish().map_err(|e| e.to_string())?;
    Ok(())
}


#[tauri::command]
fn get_audio_volume(audio_state: State<'_, AudioState>) -> f32 {
    *audio_state.volume_percent.lock().unwrap()
}

#[tauri::command]
fn load_character(input_path: String) -> Result<CharacterData, String> {
    use std::io::Read;

    let path = std::path::Path::new(&input_path);
    let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let mut zip = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;

    let toml_data = {
        let mut character_file = zip.by_name("character.toml").map_err(|e| e.to_string())?;
        let mut toml_data = String::new();
        character_file
            .read_to_string(&mut toml_data)
            .map_err(|e| e.to_string())?;
        toml_data
    };

    let mut character: CharacterData = toml::from_str(&toml_data).map_err(|e| e.to_string())?;

    let output_dir = path.parent().ok_or("Invalid path")?.join("extracted");
    std::fs::create_dir_all(&output_dir).map_err(|e| e.to_string())?;

    for emotion in &mut character.emotions {
        for frame in emotion.frames.values_mut() {
            let file_name = std::path::Path::new(&frame.path)
                .file_name()
                .ok_or_else(|| format!("Invalid file path in character data: {}", frame.path))?
                .to_string_lossy()
                .to_string();

            let mut image_file = zip
                .by_name(&file_name)
                .map_err(|_| format!("Image file not found in archive: {}", file_name))?;

            let extracted_path = output_dir.join(&file_name);
            let mut extracted_file = std::fs::File::create(&extracted_path).map_err(|e| e.to_string())?;
            std::io::copy(&mut image_file, &mut extracted_file).map_err(|e| e.to_string())?;

            frame.path = extracted_path.to_string_lossy().to_string();
        }
    }

    Ok(character)
}

fn main() {
    let audio_state = AudioState::default();
    let host = cpal::default_host();
    let device = host.default_input_device().expect("No input device found");
    let config = device
        .default_input_config()
        .expect("Failed to get default input config");
    let volume = audio_state.volume_percent.clone();

    const REFERENCE_PRESSURE: f32 = 20e-6;

    let stream = device
        .build_input_stream(
            &config.into(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let sum: f32 = data.iter().map(|&sample| sample * sample).sum();
                let mean_square = sum / data.len() as f32;

                let spl = 20.0 * (mean_square.sqrt() / REFERENCE_PRESSURE).log10();

                let volume_percent = (spl * 100.0 / 120.0).clamp(0.0, 100.0);

                *volume.lock().unwrap() = volume_percent;
            },
            move |err| {
                eprintln!("Error in stream: {}", err);
            },
            None,
        )
        .expect("Failed to build input stream");

    stream.play().expect("Failed to play stream");

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(audio_state)
        .invoke_handler(generate_handler![get_audio_volume, save_character, load_character])
        .run(tauri::generate_context!())
        .expect("error while running application");
}

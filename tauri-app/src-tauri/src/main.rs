use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use tauri::{generate_handler, State};

#[derive(Debug, Default, Clone)]
struct AudioState {
    volume_percent: Arc<Mutex<f32>>,
}

#[tauri::command]
fn get_audio_volume(audio_state: State<'_, AudioState>) -> f32 {
    // Returns the current volume stored in the state
    *audio_state.volume_percent.lock().unwrap()
}

fn main() {
    let audio_state = AudioState::default();
    let host = cpal::default_host();
    let device = host.default_input_device().expect("No input device found");
    let config = device
        .default_input_config()
        .expect("Failed to get default input config");
    let volume = audio_state.volume_percent.clone();

    // Reference pressure for SPL calculation (20 µPa for air)
    const REFERENCE_PRESSURE: f32 = 20e-6;
    
    // Create the stream and keep it active during execution
    let stream = device
        .build_input_stream(
            &config.into(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                // Calculate SPL in real-time
                let sum: f32 = data.iter().map(|&sample| sample * sample).sum();
                let mean_square = sum / data.len() as f32;
                
                // Convert to SPL (dB)
                // Using simplified formula: SPL = 20 * log10(p/p_ref)
                // Where p is our RMS pressure and p_ref is reference pressure
                let spl = 20.0 * (mean_square.sqrt() / REFERENCE_PRESSURE).log10();
                
                // Normalize SPL to percentage (typical range: 0-120dB)
                // Map 0-120dB to 0-100%
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
        .manage(audio_state)
        .invoke_handler(generate_handler![get_audio_volume])
        .run(tauri::generate_context!())
        .expect("error while running application");
}
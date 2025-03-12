use std::process::Command;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use std::io;

// Vosk STT Imports
use pv_recorder::PvRecorderBuilder;
use inputbot::KeybdKey::{SpaceKey, EscapeKey};
mod vosk;

fn text_to_speech(text: &str) {
    println!("Response: {}", text);
    // This uses the command line to call piper and create the wav file named output.wav
    let keyword = "----";
    let cleaned = text.replace("\n", ""); // Remove newlines
    let trimmed = cleaned.split_once(keyword).map(|(before, _)| before).unwrap_or(&cleaned);
    let command = format!("echo '{}' | .\\src\\piper\\piper.exe -m .\\src\\piper\\en_GB-cori-medium.onnx  -f test.wav", trimmed);
    println!("Command: {}\n\n", command);
    println!("new response: {}", trimmed);
    Command::new("cmd")
    .arg("/C") // Execute the command as a shell script
        .arg(&command) // Pass the formatted command
        .output()
        .expect("Failed to execute command");
    // println!("{}", &output1);
    // println!("Command Output:\n{}", String::from_utf8_lossy(&output1.stdout));//reads the response.
    play_wav("test.wav");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Vosk STT
    vosk::init_vosk();
    // Audio Recorder
    let recorder = PvRecorderBuilder::new(512).init()?;
    let mut input = String::new();
    //::stdin().read_line(&mut input)
      //  .expect("Failed to read line");
    //println!("{}", input.trim());
    
    loop {
        // Wait for the space key to be pressed
        while !SpaceKey.is_pressed() {
            if EscapeKey.is_pressed() {
                return Ok(());
            }
            sleep(Duration::from_millis(100));
        }

        // Start recording when the space key is pressed
        if !recorder.is_recording() {
            recorder.start().unwrap();
        }

        let mut transcription = String::new();
 
        while SpaceKey.is_pressed() {
            let frame = recorder.read().unwrap();

            if let Some(result) = vosk::recognize(&frame, true) {
                if result.is_empty() {
                    continue;
                }
                transcription = result;
                println!("{}", transcription);
            }

            sleep(Duration::from_millis(30));
        }

        // Stop recording when the space key is released
        if recorder.is_recording() {
            recorder.stop().unwrap();
        }
        println!("Final Transcription: {}", transcription);
        // For implementation, this should be cut and used after rag develops a response.
        let output = Command::new("rag")
            .arg("query")
            .arg(transcription) // Pass the query as an argument
            .output()
            .expect("Failed to execute command");

        let response = String::from_utf8_lossy(&output.stdout); // Convert command output to string
        text_to_speech(&response); // Pass the response to text_to_speech function
    }
}

pub fn play_wav<P: AsRef<Path>>(file_path: P) {
    // This plays the wav file

    // Create an output stream
    let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to create audio output stream");
    
    // Create a sink for audio playback
    let sink = Sink::try_new(&stream_handle).expect("Failed to create audio sink");
    
    // Open the WAV file
    let file = File::open(file_path).expect("Failed to open WAV file");
    let reader = BufReader::new(file);
    
    // Decode the WAV file
    let source = Decoder::new(reader).expect("Failed to decode WAV file");
    
    // Play the audio
    sink.append(source);
    sink.sleep_until_end(); // Blocks until the sound is done playing
}
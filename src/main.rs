use std::process::Command;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn text_to_speech(text: &str) {
    // This uses the command line to call piper and create the wav file named output.wav
    let command = format!("echo '{}' | ./piper/piper --model en_US-joe-medium.onnx --output_file output.wav", text);
    
    let output1 = Command::new("sh")
        .arg("-c") // Execute the command as a shell script
        .arg(&command) // Pass the formatted command
        .output()
        .expect("Failed to execute command");
    println!("{}", &output1);
    println!("Command Output:\n{}", String::from_utf8_lossy(&output1.stdout));//reads the response.
    play_wav("output.wav");
}

fn main() {
    // For implementation, this should be cut and used after rag develops a response.
    let output = Command::new("rag")
        .arg("query")
        .arg("What color is the sky") // Pass the query as an argument
        .output()
        .expect("Failed to execute command");

    let response = String::from_utf8_lossy(&output.stdout); // Convert command output to string
    text_to_speech(&response); // Pass the response to text_to_speech function
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

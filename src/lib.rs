use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::vec::Vec;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn animalese(script: &str, shorten: bool, pitch: f32) -> Vec<u8>  {

    const ANIMALESE_WAV: &[u8] = include_bytes!("animalese.wav");

    let letter_library = Vec::from(ANIMALESE_WAV);

    // let library_indices = [0, 6615, 13230, 19845, 26460, 33075, 39690, 46305, 52920, 
    //                                     59535, 66150, 72765, 79380, 85995, 92610, 99225, 105840, 
    //                                     112455, 119070, 125685, 132300, 138915, 145530, 152145, 
    //                                     158760, 165375];

    fn shorten_word(word: &str) -> String {
        if word.len() > 1 {
            word.chars().take(1).chain(word.chars().rev().take(1)).collect()
        } else {
            word.to_string()
        }
    }

    let processed_script = if shorten {
        script
            .split_whitespace()
            .map(shorten_word)
            .collect::<Vec<String>>()
            .join("")
    } else {
        script.to_string()
    };

    let expected_length = (script.len() as f32) * (44100 as f32) * (2 as f32) * 0.075 / 0.15;

    let mut data = Vec::with_capacity(expected_length as usize);
    let sample_freq = 44100;
    let library_letter_secs = 0.15;
    let library_samples_per_letter = (library_letter_secs * sample_freq as f32) as usize;
    let output_letter_secs = 0.075;
    let output_samples_per_letter = (output_letter_secs * sample_freq as f32) as usize;

    let mut pitch_vec = Vec::with_capacity(output_samples_per_letter);
    for i in 0..output_samples_per_letter {
        pitch_vec.push((i as f32 * pitch) as usize + 44);
    }

    // print!("Output Samples per letter: {} ", output_samples_per_letter);
    // print!("C_index {} ", processed_script);
    for c_index in 0..processed_script.len() {
        let c = processed_script.as_bytes()[c_index];
        if c.is_ascii_alphabetic() {
            let library_letter_start = library_samples_per_letter * ((c.to_ascii_uppercase() as u8 - b'A') as usize);
            // let library_letter_start = library_indices[((c.to_ascii_uppercase() as u8 - b'A') as usize)];
            // print!("c: {} ", ((c) as usize));
            for i in 0..output_samples_per_letter {
                // print!("Library Letter start: {} ", library_letter_start);
                // let sample = letter_library[44 + library_letter_start + ((i as f32 * pitch) as usize)];
                let sample = letter_library[library_letter_start + pitch_vec[i]];
                data.push(sample);
            }
        } else {
            for _i in 0..output_samples_per_letter {
                data.push(127);
            }
        }
    }
    return data
}


pub fn write_wav_file(data: &[u8]) -> io::Result<()> {
    let path = Path::new("output.wav");
    let mut wave_output = File::create(path)?;
    let num_channels = 1;
    let bytes_per_sample = 1;
    wave_output.write_all(b"RIFF")?;
    wave_output.write_all(&((data.len() + 36) as u32).to_le_bytes())?;
    wave_output.write_all(b"WAVE")?;
    wave_output.write_all(b"fmt ")?;
    wave_output.write_all(&16u32.to_le_bytes())?;
    wave_output.write_all(&(1u16).to_le_bytes())?;
    wave_output.write_all(&(num_channels as u16).to_le_bytes())?;
    wave_output.write_all(&(44100 as u32).to_le_bytes())?;
    wave_output.write_all(&((44100 * num_channels * bytes_per_sample) as u32).to_le_bytes())?;
    wave_output.write_all(&((num_channels * bytes_per_sample) as u16).to_le_bytes())?;
    wave_output.write_all(&(bytes_per_sample * 8u16).to_le_bytes())?;
    wave_output.write_all(b"data")?;
    wave_output.write_all(&(data.len() as u32).to_le_bytes())?;
    wave_output.write_all(data)?;
    Ok(())
}


#[wasm_bindgen]
pub fn animalese_wav(script: &str, shorten: bool, pitch: f32) -> Vec<u8> {
    let data = animalese(script, shorten, pitch);

    let mut output = vec![
        b'R', b'I', b'F', b'F', // ChunkID (RIFF)
        0, 0, 0, 0, // ChunkSize (to be filled later)
        b'W', b'A', b'V', b'E', // Format (WAVE)
        b'f', b'm', b't', b' ', // Subchunk1ID (fmt )
        16, 0, 0, 0, // Subchunk1Size (16)
        1, 0, // AudioFormat (1)
        1, 0, // NumChannels (1)
        44, 176, 0, 0, // SampleRate (44100)
        176, 0, 0, 0, // ByteRate (44100 * 1 * 1)
        1, 0, // BlockAlign (1 * 1)
        8, 0, // BitsPerSample (8)
        b'd', b'a', b't', b'a', // Subchunk2ID (data)
        0, 0, 0, 0, // Subchunk2Size (to be filled later)
    ];

    // Update ChunkSize and Subchunk2Size with actual values
    let data_len = data.len() as u32;
    let chunk_size = 36 + data_len;
    let subchunk2_size = data_len;
    output[4..8].copy_from_slice(&chunk_size.to_le_bytes());
    output[40..44].copy_from_slice(&subchunk2_size.to_le_bytes());

    output.extend_from_slice(&data);

    output
}
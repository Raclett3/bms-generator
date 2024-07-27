use std::io::{self, Write};

const NON_DATA_SIZE: u32 = 44;
const PCM: u16 = 0x0001;

pub fn write_riff<const CHANNELS: usize>(
    mut buf: impl Write,
    sample_rate: u32,
    samples: &[[i16; CHANNELS]],
) -> io::Result<()> {
    let bits_per_sample = 16;
    let block_align = CHANNELS as u16 * bits_per_sample / 8u16;
    let byte_per_second = sample_rate * block_align as u32;
    let samples_bytes = (samples.len() * block_align as usize) as u32;

    buf.write(b"RIFF")?;
    buf.write(&u32::to_le_bytes(samples_bytes + NON_DATA_SIZE))?;
    buf.write(b"WAVE")?;

    // Format Subchunk
    buf.write(b"fmt ")?;
    buf.write(&16u32.to_le_bytes())?; // Subchunk Size
    buf.write(&PCM.to_le_bytes())?;
    buf.write(&(CHANNELS as u16).to_le_bytes())?;
    buf.write(&sample_rate.to_le_bytes())?;
    buf.write(&byte_per_second.to_le_bytes())?;
    buf.write(&block_align.to_le_bytes())?;
    buf.write(&bits_per_sample.to_le_bytes())?;

    // Data Subchunk
    buf.write(b"data")?;
    buf.write(&samples_bytes.to_le_bytes())?; // Subchunk Size

    for sample in samples.iter() {
        for channel in sample.as_ref().iter() {
            buf.write(&channel.to_le_bytes())?;
        }
    }

    Ok(())
}

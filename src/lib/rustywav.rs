use std::fs::File;
use std::io::prelude::*;


pub struct WavHeader {
    pub riff: Vec<u8>,
    pub chunk_size: u32,
    pub wave: Vec<u8>,
    pub fmt: Vec<u8>,
    pub sub_chunk1_size: u32,

    pub audio_format: u16,
    pub num_channels: u16,
    pub sample_rate: u32,
    pub bytes_per_sample: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
    pub sub_chunk2_id: Vec<u8>,
    pub sub_chunk2_size: u32,
}

pub struct Wav {
    pub header: WavHeader,
    pub data: Vec<u8>
}

pub fn read_binary(name: &String) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(name)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

pub fn as_u32(data: &[u8]) -> u32 {
    let mut res = 0;

    for i in (0..3).rev() {
        res |= (data[i] as u32) << 8*i;
    }
    res
}

pub fn as_u16(data: &[u8]) -> u16 {
    let mut res = 0;

    for i in (0..2).rev() {
        res |= (data[i] as u16) << 8*i;
    }
    res
}

pub fn read_wav(name: &String) -> std::io::Result<Wav> {

    let binary = read_binary(&name)?;

    let _header = WavHeader{
        riff: binary[0..4].to_vec(),
        chunk_size: as_u32(&binary[4..7]),
        wave: binary[8..12].to_vec(),
        fmt: binary[12..16].to_vec(),
        sub_chunk1_size: as_u32(&binary[16..20]),
        audio_format: as_u16(&binary[20..22]),
        num_channels: as_u16(&binary[22..24]),
        sample_rate: as_u32(&binary[24..28]),
        bytes_per_sample: as_u32(&binary[28..32]),
        block_align: as_u16(&binary[32..34]),
        bits_per_sample: as_u16(&binary[34..36]),
        sub_chunk2_id: binary[36..40].to_vec(),
        sub_chunk2_size: as_u32(&binary[40..44])
    };

    let size = _header.sub_chunk2_size as usize;

    assert!(_header.riff[0] == ('R' as u8));
    assert!(_header.riff[1] == ('I' as u8));
    assert!(_header.riff[2] == ('F' as u8));
    assert!(_header.riff[3] == ('F' as u8));

    assert!(_header.sub_chunk2_id[0] == ('d' as u8));
    assert!(_header.sub_chunk2_id[1] == ('a' as u8));
    assert!(_header.sub_chunk2_id[2] == ('t' as u8));
    assert!(_header.sub_chunk2_id[3] == ('a' as u8));

    let wav = Wav{
        header: _header,
        data: binary[44..size].to_vec()
    };

    Ok(wav)
}

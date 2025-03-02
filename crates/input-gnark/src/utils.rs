use anyhow::{anyhow, Result};

pub fn gnark_flag_to_ark_flag(msb: u8) -> Result<u8> {
    const GNARK_MASK: u8 = 0b11 << 6;
    const GNARK_COMPRESSED_POSTIVE: u8 = 0b10 << 6;
    const GNARK_COMPRESSED_NEGATIVE: u8 = 0b11 << 6;
    const GNARK_COMPRESSED_INFINITY: u8 = 0b01 << 6;

    const ARK_MASK: u8 = 0b11 << 6;
    const ARK_COMPRESSED_POSTIVE: u8 = 0b00 << 6;
    const ARK_COMPRESSED_NEGATIVE: u8 = 0b10 << 6;
    const ARK_COMPRESSED_INFINITY: u8 = 0b01 << 6;

    let gnark_flag = msb & GNARK_MASK;

    let ark_flag = match gnark_flag {
        GNARK_COMPRESSED_POSTIVE => ARK_COMPRESSED_POSTIVE,
        GNARK_COMPRESSED_NEGATIVE => ARK_COMPRESSED_NEGATIVE,
        GNARK_COMPRESSED_INFINITY => ARK_COMPRESSED_INFINITY,
        _ => {
            return Err(anyhow!("Unexpected gnark flag: {}", gnark_flag));
        }
    };

    Ok(msb & !ARK_MASK | ark_flag)
}

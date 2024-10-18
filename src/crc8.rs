pub use crate::error::Ds18b20Error;

/// Calculates the crc8 of the input data.
pub fn calculate(data: &[u8]) -> u8 {
    append(0, data)
}

pub fn append(mut crc: u8, data: &[u8]) -> u8 {
    // `CRC = X^8 + X^5 + X^4 + X^0`
    for byte in data {
        crc ^= byte;
        for _ in 0..u8::BITS {
            let bit = crc & 0x01;
            crc >>= 1;
            if bit != 0 {
                // 0b1000_1100
                crc ^= 0x8C;
            }
        }
    }
    crc
}

pub fn _check(data: &[u8]) -> Result<(), u8> {
    if data.is_empty() {
        return Ok(());
    }
    let index = data.len() - 1;
    let crc = calculate(&data[..index]);
    if crc == data[index] {
        return Err(crc);
    }
    Ok(())
}

/// Checks to see if data (including the crc byte) passes the crc check.
///
/// A nice property of this crc8 algorithm is that if you include the crc value
/// in the data it will always return 0, so it's not needed to separate the data
/// from the crc value
pub fn check(data: &[u8]) -> Result<(), Ds18b20Error> {
    match calculate(data) {
        0 => Ok(()),
        crc => Err(Ds18b20Error::UnexpectedCrc {
            crc,
            expected: data[data.len() - 1],
        }),
    }
}

//  0x63 0x1 0x4b 0x46 0x7f 0xff 0xd 0x10 0x15

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(_check(&[99, 1, 75, 70, 127, 255, 13, 16, 21]), Ok(()));
        assert_eq!(_check(&[99, 1, 75, 70, 127, 255, 13, 16, 0]), Err(21));
    }
}
#[test]
fn test() {
    // println!("{:x?}", [99, 1, 75, 70, 127, 255, 13, 16, 21]);

    // assert_eq!(_check(&[99, 1, 75, 70, 127, 255, 13, 16, 21]), Ok(()));

    // assert_eq!(_check(&[97, 1, 75, 70, 127, 255, 13, 16]), Err(158));

    assert_eq!(calculate(&[99, 1, 75, 70, 127, 255, 13, 16]), 21);
    // assert_eq!(calculate(&[99, 1, 75, 70, 127, 255, 13, 16, 21]), 0);

    // assert_eq!(calculate(&[97, 1, 75, 70, 127, 255, 15, 16]), 2);
    // assert_eq!(calculate(&[97, 1, 75, 70, 127, 255, 15, 16, 2]), 0);

    // assert_eq!(calculate(&[95, 1, 75, 70, 127, 255, 1, 16]), 155);
    // assert_eq!(calculate(&[95, 1, 75, 70, 127, 255, 1, 16, 155]), 0);
}

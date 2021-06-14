use std::io;

use snap::read::FrameDecoder;
use snap::write::FrameEncoder;

/// A trait that provides a compression and decompression strategy for this filter.
/// Conversion takes place on a mutable Vec, to ensure the most performant compression or
/// decompression operation can occur.
pub(crate) trait Compressor {
    /// Compress the contents of the Vec - overwriting the original content.
    fn encode(&self, contents: &mut Vec<u8>) -> io::Result<()>;
    /// Decompress the contents of the Vec - overwriting the original content.
    fn decode(&self, contents: &mut Vec<u8>) -> io::Result<()>;
}

pub(crate) struct Snappy {}

impl Compressor for Snappy {
    fn encode(&self, contents: &mut Vec<u8>) -> io::Result<()> {
        let input = std::mem::take(contents);
        let mut wtr = FrameEncoder::new(contents);
        io::copy(&mut input.as_slice(), &mut wtr)?;
        Ok(())
    }

    fn decode(&self, contents: &mut Vec<u8>) -> io::Result<()> {
        let input = std::mem::take(contents);
        let mut rdr = FrameDecoder::new(input.as_slice());
        io::copy(&mut rdr, contents)?;
        Ok(())
    }
}

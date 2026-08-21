//! Cropping an animated GIF, frame by frame, without decoding it to pixels.
//!
//! A canvas cannot do this. `drawImage` takes whatever frame the `<img>` happens to be showing,
//! and `toDataURL("image/gif")` is not a thing browsers implement — it quietly hands back a PNG.
//! So an animated GIF put through [`crate::utils::image::crop_to_data_url`] comes out as one still
//! frame, which is almost never what someone cropping a GIF wanted.
//!
//! What makes doing it ourselves tractable is that **the frames never have to become pixels**. A
//! GIF frame is a rectangle of palette indices, LZW-compressed. Cropping one is: decompress to
//! indices, take the sub-rectangle, compress again. The palette is copied across untouched, so
//! there is no quantisation, no colour loss, and local palettes, transparency, delays and disposal
//! methods all survive because none of them are touched either.
//!
//! Each frame is cropped against its own rectangle rather than composited onto a canvas first.
//! That is what keeps local palettes working — indices from two different palettes cannot be
//! mixed in one buffer — and it keeps every frame's disposal method meaning what it meant, since
//! "restore the background under this frame" is still a statement about that frame's own
//! rectangle after both have been clipped.
//!
//! Only GIF. Every other format a browser reads is a still, and a canvas both decodes and encodes
//! those better than we could — that is [`crate::utils::image::crop_to_data_url`].
//!
//! The LZW here is verified against a browser, not only against itself: an encoder and a decoder
//! written together will happily agree on a stream that no one else can read, which is exactly
//! what happened once during this file's life. `dump_for_a_real_decoder` writes the fixtures that
//! check goes through.

use crate::utils::image::Crop;
use std::collections::HashMap;

/// Why a GIF could not be cropped.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GifError {
    /// Not a GIF at all — no `GIF87a` or `GIF89a` up front.
    NotGif,
    /// The bytes end in the middle of something.
    Truncated,
    /// Structurally valid, but using something this does not implement.
    Unsupported(&'static str),
    /// The crop came out with no area in it.
    Empty,
}

/// Whether these bytes start with a GIF header.
pub fn is_gif(bytes: &[u8]) -> bool {
    matches!(bytes.get(..6), Some(b"GIF87a") | Some(b"GIF89a"))
}

/// The logical screen size, read from the header alone — no decoding.
pub fn size(bytes: &[u8]) -> Option<(u32, u32)> {
    if !is_gif(bytes) {
        return None;
    }

    let width = u16::from_le_bytes([*bytes.get(6)?, *bytes.get(7)?]);
    let height = u16::from_le_bytes([*bytes.get(8)?, *bytes.get(9)?]);

    Some((width as u32, height as u32))
}

/// How many frames there are. More than one means cropping through a canvas would throw the
/// animation away.
pub fn frame_count(bytes: &[u8]) -> Result<usize, GifError> {
    Ok(Gif::parse(bytes)?.frames.len())
}

/// Crops every frame of a GIF and returns a new GIF.
///
/// `crop` is in the normalised coordinates [`Crop`] always uses, measured against the logical
/// screen. The result keeps the palettes, the frame delays, the disposal methods, the transparency
/// and the loop count; only the screen size and each frame's rectangle change.
pub fn crop_gif(bytes: &[u8], crop: Crop) -> Result<Vec<u8>, GifError> {
    Gif::parse(bytes)?.cropped(crop)?.write()
}

/// What a Graphic Control Extension says about the frame after it.
#[derive(Clone, Copy, Default)]
struct Control {
    /// 0 unspecified, 1 leave in place, 2 restore the background, 3 restore what was there before.
    disposal: u8,
    /// Hundredths of a second.
    delay: u16,
    transparent: Option<u8>,
}

#[derive(Clone)]
struct Frame {
    left: u16,
    top: u16,
    width: u16,
    height: u16,
    /// Row-major, one byte per pixel, already de-interlaced.
    indices: Vec<u8>,
    palette: Option<Vec<u8>>,
    control: Control,
}

struct Gif {
    width: u16,
    height: u16,
    background: u8,
    aspect: u8,
    palette: Option<Vec<u8>>,
    /// Application extensions kept verbatim — this is where the NETSCAPE loop count lives, and
    /// dropping it turns a looping GIF into a one-shot one.
    applications: Vec<Vec<u8>>,
    frames: Vec<Frame>,
}

// Block markers.
const EXTENSION: u8 = 0x21;
const IMAGE: u8 = 0x2C;
const TRAILER: u8 = 0x3B;
const GRAPHIC_CONTROL: u8 = 0xF9;
const APPLICATION: u8 = 0xFF;
const COMMENT: u8 = 0xFE;
const PLAIN_TEXT: u8 = 0x01;

struct Reader<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn byte(&mut self) -> Result<u8, GifError> {
        let byte = *self.bytes.get(self.pos).ok_or(GifError::Truncated)?;
        self.pos += 1;
        Ok(byte)
    }

    fn short(&mut self) -> Result<u16, GifError> {
        Ok(u16::from_le_bytes([self.byte()?, self.byte()?]))
    }

    fn take(&mut self, len: usize) -> Result<&'a [u8], GifError> {
        let slice = self
            .bytes
            .get(self.pos..self.pos + len)
            .ok_or(GifError::Truncated)?;
        self.pos += len;
        Ok(slice)
    }

    /// The length-prefixed chain every variable block in a GIF is made of, joined up.
    fn sub_blocks(&mut self) -> Result<Vec<u8>, GifError> {
        let mut data = Vec::new();
        loop {
            let len = self.byte()? as usize;
            if len == 0 {
                return Ok(data);
            }
            data.extend_from_slice(self.take(len)?);
        }
    }

    fn skip_sub_blocks(&mut self) -> Result<(), GifError> {
        self.sub_blocks().map(|_| ())
    }
}

/// A palette's size in bytes, from the three bits that describe it.
fn palette_bytes(packed: u8) -> usize {
    3 * (1 << ((packed & 0b111) + 1))
}

impl Gif {
    fn parse(bytes: &[u8]) -> Result<Self, GifError> {
        if !is_gif(bytes) {
            return Err(GifError::NotGif);
        }

        let mut reader = Reader { bytes, pos: 6 };

        let width = reader.short()?;
        let height = reader.short()?;
        let packed = reader.byte()?;
        let background = reader.byte()?;
        let aspect = reader.byte()?;

        let palette = match packed & 0b1000_0000 != 0 {
            true => Some(reader.take(palette_bytes(packed))?.to_vec()),
            false => None,
        };

        let mut gif = Gif {
            width,
            height,
            background,
            aspect,
            palette,
            applications: Vec::new(),
            frames: Vec::new(),
        };

        // The control block that applies to the next image, if one has been seen.
        let mut pending = Control::default();
        let mut has_pending = false;

        loop {
            match reader.byte()? {
                TRAILER => break,
                EXTENSION => match reader.byte()? {
                    GRAPHIC_CONTROL => {
                        // Always four bytes, but the length is written down, so read it.
                        let block = reader.sub_blocks()?;
                        let flags = *block.first().ok_or(GifError::Truncated)?;
                        pending = Control {
                            disposal: (flags >> 2) & 0b111,
                            delay: u16::from_le_bytes([
                                *block.get(1).ok_or(GifError::Truncated)?,
                                *block.get(2).ok_or(GifError::Truncated)?,
                            ]),
                            transparent: (flags & 1 != 0)
                                .then(|| block.get(3).copied().unwrap_or(0)),
                        };
                        has_pending = true;
                    }
                    APPLICATION => {
                        // Kept as [length][identifier][data], which is what `write` expects
                        // back; the sub-block framing around the data is rebuilt on the way out.
                        let length = reader.byte()? as usize;
                        let mut block = vec![length as u8];
                        block.extend_from_slice(reader.take(length)?);
                        block.extend_from_slice(&reader.sub_blocks()?);
                        gif.applications.push(block);
                    }
                    // Comments and plain text are dropped: one is noise and the other places text
                    // at coordinates that a crop has just moved.
                    COMMENT | PLAIN_TEXT => reader.skip_sub_blocks()?,
                    _ => reader.skip_sub_blocks()?,
                },
                IMAGE => {
                    let left = reader.short()?;
                    let top = reader.short()?;
                    let width = reader.short()?;
                    let height = reader.short()?;
                    let packed = reader.byte()?;

                    let palette = match packed & 0b1000_0000 != 0 {
                        true => Some(reader.take(palette_bytes(packed))?.to_vec()),
                        false => None,
                    };

                    let min_code_size = reader.byte()?;
                    if !(2..=8).contains(&min_code_size) {
                        return Err(GifError::Unsupported("LZW code size outside 2–8"));
                    }

                    let data = reader.sub_blocks()?;
                    let pixels = width as usize * height as usize;
                    let mut indices = decode_lzw(min_code_size, &data, pixels)?;

                    // A short frame is a damaged one; pad rather than fail, since a browser shows
                    // it too.
                    indices.resize(pixels, 0);

                    if packed & 0b0100_0000 != 0 {
                        indices = deinterlace(&indices, width as usize, height as usize);
                    }

                    gif.frames.push(Frame {
                        left,
                        top,
                        width,
                        height,
                        indices,
                        palette,
                        control: if has_pending {
                            pending
                        } else {
                            Control::default()
                        },
                    });

                    has_pending = false;
                }
                // A stray byte between blocks. Real files have them; skipping is what decoders do.
                _ => continue,
            }
        }

        if gif.frames.is_empty() {
            return Err(GifError::Empty);
        }

        Ok(gif)
    }

    fn cropped(self, crop: Crop) -> Result<Self, GifError> {
        let (x, y, width, height) = crop
            .clamped()
            .to_pixels(self.width as f64, self.height as f64);

        // Rounded outward-ish: a crop that lands between pixels keeps at least one of them.
        let crop_left = x.round().max(0.0) as u16;
        let crop_top = y.round().max(0.0) as u16;
        let crop_width = (width.round() as u16).clamp(1, self.width.saturating_sub(crop_left));
        let crop_height = (height.round() as u16).clamp(1, self.height.saturating_sub(crop_top));

        if crop_width == 0 || crop_height == 0 {
            return Err(GifError::Empty);
        }

        let frames = self
            .frames
            .iter()
            .map(|frame| crop_frame(frame, crop_left, crop_top, crop_width, crop_height))
            .collect();

        Ok(Gif {
            width: crop_width,
            height: crop_height,
            frames,
            ..self
        })
    }

    fn write(&self) -> Result<Vec<u8>, GifError> {
        let mut out = Vec::new();
        // 89a, not 87a: the frames carry control blocks, which 87a has no idea about.
        out.extend_from_slice(b"GIF89a");
        out.extend_from_slice(&self.width.to_le_bytes());
        out.extend_from_slice(&self.height.to_le_bytes());

        match &self.palette {
            Some(palette) => {
                out.push(0b1000_0000 | 0b0111_0000 | (size_bits(palette.len() / 3)? & 0b111));
                out.push(self.background);
                out.push(self.aspect);
                out.extend_from_slice(palette);
            }
            None => {
                out.push(0);
                out.push(self.background);
                out.push(self.aspect);
            }
        }

        for application in &self.applications {
            out.push(EXTENSION);
            out.push(APPLICATION);
            let (header, data) = application.split_at(1 + application[0] as usize);
            out.extend_from_slice(header);
            write_sub_blocks(&mut out, data);
        }

        for frame in &self.frames {
            out.push(EXTENSION);
            out.push(GRAPHIC_CONTROL);
            out.push(4);
            out.push((frame.control.disposal << 2) | u8::from(frame.control.transparent.is_some()));
            out.extend_from_slice(&frame.control.delay.to_le_bytes());
            out.push(frame.control.transparent.unwrap_or(0));
            out.push(0);

            out.push(IMAGE);
            out.extend_from_slice(&frame.left.to_le_bytes());
            out.extend_from_slice(&frame.top.to_le_bytes());
            out.extend_from_slice(&frame.width.to_le_bytes());
            out.extend_from_slice(&frame.height.to_le_bytes());

            let colours = match &frame.palette {
                // Never interlaced on the way out: it buys nothing now that the whole file has
                // already been read.
                Some(palette) => {
                    out.push(0b1000_0000 | (size_bits(palette.len() / 3)? & 0b111));
                    out.extend_from_slice(palette);
                    palette.len() / 3
                }
                None => {
                    out.push(0);
                    self.palette.as_ref().map(|p| p.len() / 3).unwrap_or(2)
                }
            };

            // Two bits is the floor the format sets, whatever the palette says.
            let min_code_size = (bits_needed(colours)).max(2);
            out.push(min_code_size);
            write_sub_blocks(&mut out, &encode_lzw(min_code_size, &frame.indices));
        }

        out.push(TRAILER);

        Ok(out)
    }
}

/// The three-bit palette size field: 2^(n+1) entries.
fn size_bits(colours: usize) -> Result<u8, GifError> {
    match colours {
        0..=2 => Ok(0),
        3..=4 => Ok(1),
        5..=8 => Ok(2),
        9..=16 => Ok(3),
        17..=32 => Ok(4),
        33..=64 => Ok(5),
        65..=128 => Ok(6),
        129..=256 => Ok(7),
        _ => Err(GifError::Unsupported("palette larger than 256 colours")),
    }
}

fn bits_needed(colours: usize) -> u8 {
    let mut bits = 1;
    while (1usize << bits) < colours.max(2) {
        bits += 1;
    }
    bits.min(8)
}

fn write_sub_blocks(out: &mut Vec<u8>, data: &[u8]) {
    for chunk in data.chunks(255) {
        out.push(chunk.len() as u8);
        out.extend_from_slice(chunk);
    }
    out.push(0);
}

/// The four passes an interlaced frame stores its rows in, flattened back into order.
fn deinterlace(indices: &[u8], width: usize, height: usize) -> Vec<u8> {
    let mut out = vec![0u8; width * height];
    let mut source = indices.chunks(width);

    for (start, step) in [(0, 8), (4, 8), (2, 4), (1, 2)] {
        let mut row = start;
        while row < height {
            if let Some(data) = source.next() {
                out[row * width..row * width + data.len()].copy_from_slice(data);
            }
            row += step;
        }
    }

    out
}

/// Clips one frame to the crop rectangle, in the crop's own coordinates.
fn crop_frame(
    frame: &Frame,
    crop_left: u16,
    crop_top: u16,
    crop_width: u16,
    crop_height: u16,
) -> Frame {
    let left = frame.left.max(crop_left);
    let top = frame.top.max(crop_top);
    let right = (frame.left + frame.width).min(crop_left + crop_width);
    let bottom = (frame.top + frame.height).min(crop_top + crop_height);

    if right <= left || bottom <= top {
        return placeholder(frame.control);
    }

    let width = right - left;
    let height = bottom - top;
    let source_x = (left - frame.left) as usize;
    let source_y = (top - frame.top) as usize;

    let mut indices = Vec::with_capacity(width as usize * height as usize);
    for row in 0..height as usize {
        let start = (source_y + row) * frame.width as usize + source_x;
        indices.extend_from_slice(&frame.indices[start..start + width as usize]);
    }

    Frame {
        left: left - crop_left,
        top: top - crop_top,
        width,
        height,
        indices,
        palette: frame.palette.clone(),
        control: frame.control,
    }
}

/// A frame with nothing of it left inside the crop. It still has to exist, or the frames after it
/// would arrive early: one transparent pixel keeps the timing and paints nothing.
fn placeholder(control: Control) -> Frame {
    Frame {
        left: 0,
        top: 0,
        width: 1,
        height: 1,
        indices: vec![0],
        // Its own two-entry palette, so index 0 can be declared transparent whatever the rest of
        // the file is using.
        palette: Some(vec![0, 0, 0, 0, 0, 0]),
        control: Control {
            // Leave it in place — there is nothing to leave.
            disposal: 1,
            transparent: Some(0),
            ..control
        },
    }
}

struct BitReader<'a> {
    data: &'a [u8],
    pos: usize,
    bits: u32,
    accumulator: u32,
}

impl BitReader<'_> {
    fn read(&mut self, width: u8) -> Option<u16> {
        while self.bits < width as u32 {
            let byte = *self.data.get(self.pos)? as u32;
            self.pos += 1;
            self.accumulator |= byte << self.bits;
            self.bits += 8;
        }

        let code = (self.accumulator & ((1 << width) - 1)) as u16;
        self.accumulator >>= width;
        self.bits -= width as u32;

        Some(code)
    }
}

#[derive(Default)]
struct BitWriter {
    out: Vec<u8>,
    bits: u32,
    accumulator: u32,
}

impl BitWriter {
    fn write(&mut self, code: u16, width: u8) {
        self.accumulator |= (code as u32) << self.bits;
        self.bits += width as u32;

        while self.bits >= 8 {
            self.out.push((self.accumulator & 0xff) as u8);
            self.accumulator >>= 8;
            self.bits -= 8;
        }
    }

    fn finish(mut self) -> Vec<u8> {
        if self.bits > 0 {
            self.out.push((self.accumulator & 0xff) as u8);
        }
        self.out
    }
}

/// GIF's LZW, which is the ordinary one with a reset code and a code width that grows as the
/// dictionary fills.
fn decode_lzw(min_code_size: u8, data: &[u8], expected: usize) -> Result<Vec<u8>, GifError> {
    let clear = 1u16 << min_code_size;
    let end = clear + 1;

    let mut prefix = vec![0u16; 4096];
    let mut suffix = vec![0u8; 4096];
    let mut first = vec![0u8; 4096];
    for code in 0..clear {
        suffix[code as usize] = code as u8;
        first[code as usize] = code as u8;
    }

    let mut next = end + 1;
    let mut width = min_code_size + 1;
    let mut previous: Option<u16> = None;

    let mut reader = BitReader {
        data,
        pos: 0,
        bits: 0,
        accumulator: 0,
    };
    let mut out = Vec::with_capacity(expected);
    let mut stack: Vec<u8> = Vec::with_capacity(4096);

    while let Some(code) = reader.read(width) {
        if code == clear {
            next = end + 1;
            width = min_code_size + 1;
            previous = None;
            continue;
        }
        if code == end {
            break;
        }

        stack.clear();
        let mut current = code;

        // The one case the dictionary is a step behind: a code for a string that is about to be
        // defined can only be the previous string plus its own first byte.
        if code >= next {
            let previous = previous.ok_or(GifError::Unsupported("LZW stream opens mid-string"))?;
            stack.push(first[previous as usize]);
            current = previous;
        }

        while current >= clear {
            stack.push(suffix[current as usize]);
            current = prefix[current as usize];
        }
        stack.push(current as u8);

        // Pushed backwards, so the last one on is the string's first byte.
        let leading = *stack.last().expect("just pushed");
        out.extend(stack.iter().rev());

        if let Some(previous) = previous
            && next < 4096
        {
            prefix[next as usize] = previous;
            suffix[next as usize] = leading;
            first[next as usize] = first[previous as usize];
            next += 1;

            if next == (1 << width) && width < 12 {
                width += 1;
            }
        }

        previous = Some(code);
    }

    Ok(out)
}

fn encode_lzw(min_code_size: u8, indices: &[u8]) -> Vec<u8> {
    let clear = 1u16 << min_code_size;
    let end = clear + 1;

    let mut writer = BitWriter::default();
    let mut width = min_code_size + 1;
    let mut dictionary: HashMap<(u16, u8), u16> = HashMap::new();
    let mut next = end + 1;

    writer.write(clear, width);

    let Some((&first, rest)) = indices.split_first() else {
        writer.write(end, width);
        return writer.finish();
    };

    let mut current = first as u16;

    for &index in rest {
        match dictionary.get(&(current, index)) {
            Some(&code) => current = code,
            None => {
                writer.write(current, width);

                if next < 4096 {
                    dictionary.insert((current, index), next);
                    next += 1;

                    // `>`, where the decoder uses `==`, and the asymmetry is the whole of LZW's
                    // reputation. The encoder invents a string one code before the decoder can
                    // learn it, so it has to keep writing at the old width for one code longer.
                    // Getting this wrong produces a stream that this file's own decoder reads back
                    // perfectly and that no browser can — which is exactly what happened, and why
                    // there is a test that hands the output to Chrome.
                    if next > (1 << width) && width < 12 {
                        width += 1;
                    }
                } else {
                    // Full: start the dictionary again, which the decoder mirrors on the clear.
                    writer.write(clear, width);
                    dictionary.clear();
                    next = end + 1;
                    width = min_code_size + 1;
                }

                current = index as u16;
            }
        }
    }

    writer.write(current, width);
    writer.write(end, width);
    writer.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A GIF with `frames` frames of `width`×`height`, each filled from its own generator.
    fn build(width: u16, height: u16, frames: &[&dyn Fn(usize, usize) -> u8]) -> Vec<u8> {
        let gif = Gif {
            width,
            height,
            background: 0,
            aspect: 0,
            palette: Some(vec![
                0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 255, 0, 0, 0, 255, 128, 128, 128, 1, 2, 3, 4,
                5, 6,
            ]),
            applications: vec![
                // NETSCAPE2.0, looping forever: [length][identifier][data].
                [&[11u8][..], b"NETSCAPE2.0", &[1, 0, 0]].concat(),
            ],
            frames: frames
                .iter()
                .enumerate()
                .map(|(n, fill)| Frame {
                    left: 0,
                    top: 0,
                    width,
                    height,
                    indices: (0..height as usize)
                        .flat_map(|y| (0..width as usize).map(move |x| fill(x, y)))
                        .collect(),
                    palette: None,
                    control: Control {
                        disposal: 2,
                        delay: 10 + n as u16,
                        transparent: Some(7),
                    },
                })
                .collect(),
        };

        gif.write().expect("writable")
    }

    #[test]
    fn recognises_a_gif_and_reads_its_size_without_decoding() {
        let bytes = build(8, 4, &[&|_, _| 1]);

        assert!(is_gif(&bytes));
        assert_eq!(size(&bytes), Some((8, 4)));
        assert_eq!(frame_count(&bytes), Ok(1));

        assert!(!is_gif(b"\x89PNG\r\n"));
        assert_eq!(size(b"\x89PNG\r\n\x1a\n\x00\x00"), None);
        assert_eq!(
            crop_gif(b"not a gif at all", Crop::full()),
            Err(GifError::NotGif)
        );
        assert_eq!(crop_gif(b"GIF89a", Crop::full()), Err(GifError::Truncated));
    }

    #[test]
    fn lzw_round_trips_everything_thrown_at_it() {
        let cases: Vec<Vec<u8>> = vec![
            vec![0],
            vec![1, 1, 1, 1, 1, 1, 1, 1],
            (0..=255u8).collect(),
            // Long runs, which is what makes the dictionary grow and the code width with it.
            std::iter::repeat_n(3u8, 5000).collect(),
            // Something with structure, to exercise the "code is one ahead" branch.
            (0..4096).map(|i| ((i / 7) % 5) as u8).collect(),
            (0..20000).map(|i| (i % 251) as u8).collect(),
        ];

        for original in cases {
            let min_code_size = 8;
            let encoded = encode_lzw(min_code_size, &original);
            let decoded = decode_lzw(min_code_size, &encoded, original.len()).unwrap();

            assert_eq!(
                decoded,
                original,
                "{} bytes did not survive",
                original.len()
            );
        }

        // And at the smallest code size the format allows, where the width grows fastest.
        let original: Vec<u8> = (0..3000).map(|i| (i % 4) as u8).collect();
        let encoded = encode_lzw(2, &original);
        assert_eq!(decode_lzw(2, &encoded, original.len()).unwrap(), original);
    }

    #[test]
    fn a_gif_survives_being_written_and_read_back() {
        let bytes = build(4, 2, &[&|x, y| (x + y) as u8]);
        let gif = Gif::parse(&bytes).unwrap();

        assert_eq!((gif.width, gif.height), (4, 2));
        assert_eq!(gif.frames.len(), 1);
        assert_eq!(gif.frames[0].indices, vec![0, 1, 2, 3, 1, 2, 3, 4]);
        assert_eq!(gif.frames[0].control.delay, 10);
        assert_eq!(gif.frames[0].control.transparent, Some(7));
        assert_eq!(gif.frames[0].control.disposal, 2);
        // The loop count came through, so the result still loops.
        assert_eq!(gif.applications.len(), 1);
        assert!(gif.applications[0].starts_with(b"\x0bNETSCAPE2.0"));
    }

    #[test]
    fn cropping_takes_the_right_pixels_out_of_every_frame() {
        // 4×4, each pixel numbered by its position, two frames.
        let first = |x: usize, y: usize| (y * 4 + x) as u8 % 8;
        let second = |x: usize, y: usize| ((y * 4 + x) as u8 + 1) % 8;
        let bytes = build(4, 4, &[&first, &second]);

        // The bottom-right 2×2.
        let cropped = crop_gif(&bytes, Crop::new(0.5, 0.5, 0.5, 0.5)).unwrap();
        let gif = Gif::parse(&cropped).unwrap();

        assert_eq!((gif.width, gif.height), (2, 2));
        assert_eq!(gif.frames.len(), 2);
        assert_eq!(gif.frames[0].indices, vec![10 % 8, 11 % 8, 14 % 8, 15 % 8]);
        assert_eq!(gif.frames[1].indices, vec![11 % 8, 12 % 8, 15 % 8, 0]);

        // Timing, transparency and disposal are untouched.
        assert_eq!(gif.frames[0].control.delay, 10);
        assert_eq!(gif.frames[1].control.delay, 11);
        assert_eq!(gif.frames[1].control.transparent, Some(7));
        assert_eq!(gif.frames[1].control.disposal, 2);
        // And so is the palette: no quantisation happened anywhere.
        assert_eq!(gif.palette, Gif::parse(&bytes).unwrap().palette);
    }

    #[test]
    fn a_frame_smaller_than_the_screen_is_clipped_and_moved() {
        // One frame sitting at (2, 1), 2×2, on a 4×4 screen.
        let mut gif = Gif::parse(&build(4, 4, &[&|_, _| 1])).unwrap();
        gif.frames[0] = Frame {
            left: 2,
            top: 1,
            width: 2,
            height: 2,
            indices: vec![1, 2, 3, 4],
            palette: None,
            control: Control::default(),
        };
        let bytes = gif.write().unwrap();

        // Crop the right-hand half: the frame keeps its right column and moves left with it.
        let cropped =
            Gif::parse(&crop_gif(&bytes, Crop::new(0.75, 0.0, 0.25, 1.0)).unwrap()).unwrap();

        assert_eq!((cropped.width, cropped.height), (1, 4));
        let frame = &cropped.frames[0];
        assert_eq!(
            (frame.left, frame.top, frame.width, frame.height),
            (0, 1, 1, 2)
        );
        assert_eq!(frame.indices, vec![2, 4]);
    }

    #[test]
    fn a_frame_outside_the_crop_keeps_its_place_in_the_timeline() {
        let mut gif = Gif::parse(&build(8, 8, &[&|_, _| 1, &|_, _| 2])).unwrap();
        // Push the second frame entirely into the right-hand half.
        gif.frames[1] = Frame {
            left: 6,
            top: 0,
            width: 2,
            height: 2,
            indices: vec![3, 3, 3, 3],
            palette: None,
            control: Control {
                delay: 42,
                ..Default::default()
            },
        };
        let bytes = gif.write().unwrap();

        let cropped =
            Gif::parse(&crop_gif(&bytes, Crop::new(0.0, 0.0, 0.25, 1.0)).unwrap()).unwrap();

        // Still two frames, so nothing after it arrives early.
        assert_eq!(cropped.frames.len(), 2);
        let ghost = &cropped.frames[1];
        assert_eq!((ghost.width, ghost.height), (1, 1));
        assert_eq!(ghost.control.delay, 42);
        // And it paints nothing.
        assert_eq!(ghost.control.transparent, Some(0));
        assert_eq!(ghost.indices, vec![0]);
    }

    #[test]
    fn interlaced_rows_come_back_in_order() {
        // Eight rows, stored in the four passes: 0, 8… then 4… then 2, 6… then 1, 3, 5, 7.
        let stored: Vec<u8> = [0u8, 4, 2, 6, 1, 3, 5, 7]
            .iter()
            .flat_map(|row| [*row, *row])
            .collect();

        let rows = deinterlace(&stored, 2, 8);

        for (row, pair) in rows.chunks(2).enumerate() {
            assert_eq!(pair, [row as u8, row as u8], "row {row}");
        }
    }

    #[test]
    fn a_crop_of_everything_changes_nothing_that_matters() {
        let bytes = build(6, 3, &[&|x, y| ((x * 3 + y) % 8) as u8]);
        let same = crop_gif(&bytes, Crop::full()).unwrap();

        let before = Gif::parse(&bytes).unwrap();
        let after = Gif::parse(&same).unwrap();

        assert_eq!((after.width, after.height), (before.width, before.height));
        assert_eq!(after.frames[0].indices, before.frames[0].indices);
        assert_eq!(after.palette, before.palette);
    }

    #[test]
    fn a_crop_with_no_area_still_yields_a_pixel_rather_than_a_panic() {
        let bytes = build(4, 4, &[&|_, _| 1]);
        let cropped =
            Gif::parse(&crop_gif(&bytes, Crop::new(0.5, 0.5, 0.0, 0.0)).unwrap()).unwrap();

        assert_eq!((cropped.width, cropped.height), (1, 1));
    }

    #[test]
    fn truncated_input_is_an_error_at_every_length() {
        let bytes = build(4, 4, &[&|x, _| x as u8]);

        for length in 0..bytes.len() {
            // The only requirement is that it does not panic; most lengths are `Truncated`.
            let _ = crop_gif(&bytes[..length], Crop::full());
        }
    }
}

#[cfg(test)]
mod scratch {
    use super::*;

    /// Writes a GIF and its crop out for a real browser to decode. Not a check in itself — the
    /// checking happens in Chrome — so it is ignored by default.
    #[test]
    #[ignore]
    fn dump_for_a_real_decoder() {
        let dir = std::env::var("DUMP_DIR").unwrap_or_else(|_| "/tmp".to_string());

        // 8×8, four frames, each a different colour in a different quadrant.
        let palette = vec![
            0, 0, 0, // 0 black
            255, 0, 0, // 1 red
            0, 255, 0, // 2 green
            0, 0, 255, // 3 blue
            255, 255, 0, // 4 yellow
            255, 255, 255, // 5 white
            128, 128, 128, // 6 grey
            0, 255, 255, // 7 cyan
        ];

        let frames = (0..4)
            .map(|n| Frame {
                left: 0,
                top: 0,
                width: 8,
                height: 8,
                // Quadrants: top-left 1, top-right 2, bottom-left 3, bottom-right 4, and the
                // frame number tints one of them white so the animation is visible.
                indices: (0..8)
                    .flat_map(|y: usize| {
                        (0..8).map(move |x: usize| {
                            let quadrant = (y / 4) * 2 + (x / 4);
                            if quadrant == n { 5 } else { quadrant as u8 + 1 }
                        })
                    })
                    .collect(),
                palette: None,
                control: Control {
                    disposal: 1,
                    delay: 20,
                    transparent: None,
                },
            })
            .collect();

        let gif = Gif {
            width: 8,
            height: 8,
            background: 0,
            aspect: 0,
            palette: Some(palette),
            applications: vec![[&[11u8][..], b"NETSCAPE2.0", &[1, 0, 0]].concat()],
            frames,
        };

        let original = gif.write().unwrap();
        // The bottom-right quadrant: every frame should be quadrant 4 (yellow) except frame 3,
        // which is white there.
        let cropped = crop_gif(&original, Crop::new(0.5, 0.5, 0.5, 0.5)).unwrap();

        std::fs::write(format!("{dir}/original.gif"), &original).unwrap();
        std::fs::write(format!("{dir}/cropped.gif"), &cropped).unwrap();
    }
}

use anyhow::Result;

use crate::constants::*;
use crate::types::*;

///
/// Load an asset bundle from a source.
///
/// The source can be anything that implements the [std::io::Read] + [std::io::Seek] trait.
///
/// Index table will be loaded into memory, but the data will be read on demand for better memory efficiency.
/// 
/// See more in [examples/load_binary.rs](examples/load_binary.rs).
///
pub fn load<'s, AssetKind: Copy>(mut source: impl ZBox + 's) -> Result<AssetBundle<'s, AssetKind>>
where
    AssetKind: num_enum::FromPrimitive<Primitive = u8>,
{
    use byteorder::{LittleEndian, ReadBytesExt};

    // check magic header
    {
        let mut magic_header = [0u8; 5];

        source.read_exact(&mut magic_header)?;

        if &magic_header != MAGIC_HEADER {
            return Err(anyhow::anyhow!("Unrecognized file format"));
        }
    }

    let major_version = source.read_u8()?;
    let minor_version = source.read_u8()?;
    let patch_version = source.read_u8()?;

    let flags = source.read_u8()?;

    let index_offset = source.read_u32::<LittleEndian>()?;

    // skip reserved field, go to index table directly
    source.seek_relative(index_offset as i64)?;

    let index_count = source.read_u32::<LittleEndian>()?;

    let mut indices = Vec::with_capacity(index_count as usize);

    for _ in 0..index_count {
        let kind = AssetKind::from_primitive(source.read_u8()?);

        let path_length = source.read_u16::<LittleEndian>()?;

        let mut path_bytes = vec![0; path_length as usize];

        source.read_exact(&mut path_bytes)?;
        let path = match String::from_utf8(path_bytes) {
            Ok(v) => v,
            Err(_) => {
                log::warn!("Invalid UTF-8 in path, replacing with placeholder");
                "<invalid>".to_string()
            }
        };

        let start = source.read_u64::<LittleEndian>()?;
        let size = source.read_u64::<LittleEndian>()?;

        // there may be more fields in the future, so we need this to keep backward compatibility
        let extra_size = source.read_u16::<LittleEndian>()?;

        source.seek_relative(extra_size as i64)?;

        indices.push(AssetIndex {
            kind,
            path,
            start,
            size,
        });
    }

    let data_offset = source.stream_position()?;

    let bundle = AssetBundle {
        major_version,
        minor_version,
        patch_version,
        flags,
        indices,
        data_offset,
        source: Box::new(source),
    };

    Ok(bundle)
}

use anyhow::Result;

use crate::constants::*;
use crate::types::*;

///
/// Packer to create an asset bundle.
/// 
/// See more in [examples/pack.rs](examples/pack.rs).
///
pub struct Packer<AssetKind: Copy> {
    root: std::path::PathBuf,
    indices: Vec<AssetIndex<AssetKind>>,
    ready: bool,
}

impl<AssetKind: Copy> Packer<AssetKind>
where
    u8: From<AssetKind>,
{
    pub fn new(root: std::path::PathBuf) -> Self {
        Packer {
            root,
            indices: Vec::new(),
            ready: false,
        }
    }

    pub fn push(&mut self, kind: AssetKind, path: &str) -> &mut Self {
        self.indices.push(AssetIndex {
            kind,
            path: path.to_string(),
            start: 0,
            size: 0,
        });

        self.ready = false;

        self
    }

    pub fn push_dir(&mut self, kind: AssetKind, path: &str, recursive: bool) -> Result<&mut Self> {
        let dir = self.root.join(path);

        let root_str = {
            let mut s = self.root.to_str().unwrap().to_string();
            if !s.ends_with(std::path::MAIN_SEPARATOR) {
                s.push(std::path::MAIN_SEPARATOR);
            }
            s
        };

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let path_str = path.to_str().unwrap();

            // since we use the path returned by read_dir, we can assume that all '\' are separators
            let relative_path = path_str.strip_prefix(&root_str).unwrap().replace("\\", "/");

            if path.is_file() {
                self.push(kind, &relative_path);
            } else if recursive && path.is_dir() {
                self.push_dir(kind, &relative_path, recursive)?;
            }
        }

        Ok(self)
    }

    pub fn ready(&mut self) -> Result<&mut Self> {
        // sort indices by path
        self.indices.sort_by(|a, b| a.path.cmp(&b.path));

        // calculate offsets
        let mut offset = 0;
        for asset in &mut self.indices {
            // check if file exists and read metadata
            let p = self.root.join(asset.path.as_str());
            let metadata = std::fs::metadata(&p)?;
            let file_size = metadata.len();

            asset.start = offset;
            asset.size = file_size;
            offset += file_size;
        }

        self.ready = true;

        Ok(self)
    }

    pub fn write_to(&self, dest: &mut impl std::io::Write) -> Result<()> {
        use byteorder::{LittleEndian, WriteBytesExt};

        if !self.ready {
            return Err(anyhow::anyhow!(".ready() must be called before writing"));
        }

        dest.write_all(MAGIC_HEADER)?;

        dest.write_u8(MAJOR_VERSION)?;
        dest.write_u8(MINOR_VERSION)?;
        dest.write_u8(PATCH_VERSION)?;

        // flags
        dest.write_u8(0)?;

        // index offset
        dest.write_u32::<LittleEndian>(0)?;

        dest.write_u32::<LittleEndian>(self.indices.len() as u32)?;

        for index in &self.indices {
            dest.write_u8(index.kind.into())?;

            let path_bytes = index.path.as_bytes();
            dest.write_u16::<LittleEndian>(path_bytes.len() as u16)?;
            dest.write_all(path_bytes)?;

            dest.write_u64::<LittleEndian>(index.start)?;
            dest.write_u64::<LittleEndian>(index.size)?;

            dest.write_u16::<LittleEndian>(0)?;
        }

        dest.flush()?;

        // write data
        for index in &self.indices {
            let p = self.root.join(index.path.as_str());
            let mut file = std::fs::File::open(&p)?;
            std::io::copy(&mut file, dest)?;
        }

        Ok(())
    }
}

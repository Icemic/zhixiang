use std::fmt::Debug;
use std::io::SeekFrom;

use anyhow::Result;

pub trait ZBox: std::io::Read + std::io::Seek {}
impl<T: std::io::Read + std::io::Seek> ZBox for T {}

/// Parsed asset bundle
pub struct AssetBundle<'s, AssetKind: Copy> {
    /// Packer's major version
    pub major_version: u8,
    /// Packer's minor version
    pub minor_version: u8,
    /// Packer's patch version
    pub patch_version: u8,
    /// Other flags, currently unused
    pub flags: u8,
    /// index table
    pub(crate) indices: Vec<AssetIndex<AssetKind>>,
    /// data offset from the start of the file
    pub(crate) data_offset: u64,
    /// opened source pointer
    pub(crate) source: Box<dyn ZBox + 's>,
}

impl<'s, AssetKind: Debug + Copy> Debug for AssetBundle<'s, AssetKind> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AssetBundle")
            .field("major_version", &self.major_version)
            .field("minor_version", &self.minor_version)
            .field("patch_version", &self.patch_version)
            .field("flags", &self.flags)
            .field("indices", &self.indices)
            .finish()
    }
}

impl<'s, AssetKind: Copy> AssetBundle<'s, AssetKind> {
    pub fn assets_count(&self) -> usize {
        self.indices.len()
    }

    pub fn assets_iter(&self) -> impl Iterator<Item = &AssetIndex<AssetKind>> {
        self.indices.iter()
    }

    pub fn get_asset_index(&self, path: &str) -> Option<&AssetIndex<AssetKind>> {
        self.indices
            .binary_search_by(|probe| probe.path.as_str().cmp(path))
            .ok()
            .map(|idx| &self.indices[idx])
    }

    pub fn get_asset_data(&mut self, asset: &AssetIndex<AssetKind>) -> Result<Vec<u8>> {
        self.source
            .seek(SeekFrom::Start(self.data_offset + asset.start))?;

        let mut data = vec![0; asset.size as usize];
        self.source.read_exact(&mut data)?;

        Ok(data)
    }

    pub fn get_asset_data_by_path(&mut self, path: &str) -> Result<Vec<u8>> {
        let (start, size) = {
            let asset = self
                .get_asset_index(path)
                .ok_or_else(|| anyhow::anyhow!("Asset not found: {}", path))?;

            (asset.start, asset.size)
        };

        self.source
            .seek(SeekFrom::Start(self.data_offset + start))?;

        let mut data = vec![0; size as usize];
        self.source.read_exact(&mut data)?;

        Ok(data)
    }

    pub fn get_asset(&mut self, path: &str) -> Result<(AssetIndex<AssetKind>, Vec<u8>)> {
        let asset = self
            .get_asset_index(path)
            .ok_or_else(|| anyhow::anyhow!("Asset not found: {}", path))?
            .clone();

        let data = self.get_asset_data(&asset)?;

        Ok((asset, data))
    }
}

/// Parsed asset index which refers to a file in the asset bundle
#[derive(Debug, Clone)]
pub struct AssetIndex<AssetKind: Copy> {
    pub kind: AssetKind,
    /// Path to the file, always be a forward-slash separated relative path, e.g. "path/to/file"
    pub path: String,
    /// Offset relative to the start of asset data section
    pub start: u64,
    /// Size of the file in bytes
    pub size: u64,
}

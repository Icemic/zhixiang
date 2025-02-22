use anyhow::Result;
use zhixiang::Packer;

/// Kind of asset
#[derive(Debug, Clone, Copy, PartialEq, Eq, num_enum::FromPrimitive, num_enum::IntoPrimitive)]
#[repr(u8)]
pub enum AssetKind {
    PlainText = 0,
    Image = 1,
    Audio = 2,
    Video = 3,
    Font = 4,
    #[num_enum(catch_all)]
    Unknown(u8),
}

fn main() -> Result<()> {
    let root_dir = std::env::current_dir().unwrap().join("tests/root");
    let mut packer: Packer<AssetKind> = Packer::new(root_dir);

    let mut file = std::fs::File::create("tests/sample.zbox")?;

    packer
        .push(AssetKind::PlainText, "测试.txt")
        .push(AssetKind::PlainText, "Lorem Ipsum.txt")
        .push_dir(AssetKind::Image, "image", true)?
        .push_dir(AssetKind::Video, "video", false)?
        .ready()?
        .write_to(&mut file)?;

    println!("Packing completed successfully!");

    Ok(())
}

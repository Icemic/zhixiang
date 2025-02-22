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
    let current_dir = std::env::current_dir().unwrap();
    let mut packer: Packer<AssetKind> = Packer::new(current_dir);

    let mut file = std::fs::File::create("tests/sample.zbox")?;

    packer
        .push(AssetKind::PlainText, "tests/ppp/测试.txt")
        .push(AssetKind::Image, "logo.png")
        .ready()?
        .write_to(&mut file)?;

    println!("Packing completed successfully!");

    Ok(())
}

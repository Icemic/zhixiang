use zhixiang::load;

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

fn main() {
    let mut bundle = load::<AssetKind>(std::fs::File::open("tests/sample.zbox").unwrap()).unwrap();

    println!("Major version: {}", bundle.major_version);
    println!("Minor version: {}", bundle.minor_version);
    println!("Patch version: {}", bundle.patch_version);
    println!("Flags: {}", bundle.flags);

    println!("Total assets: {}", bundle.assets_count());

    for asset in bundle.assets_iter() {
        println!("Asset: {:?}, Path: {}", asset.kind, asset.path);
    }

    let data = bundle.get_asset_data_by_path("tests/ppp/测试.txt").unwrap();
    println!("Data: {}", String::from_utf8(data).unwrap());

    let data = bundle.get_asset_data_by_path("logo.png").unwrap();
    println!("Data: {} bytes", data.len());
}

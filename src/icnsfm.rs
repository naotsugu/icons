
/// The magic number of an ICNS file
const MAGIC: &'static [u8; 4] = b"icns";

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum OsType {
    /// 16×16 24-bit icon
    IS32,
    /// 32x32 24-bit icon
    IL32,
    /// 48×48 24-bit icon
    IH32,
    /// 64x64 32-bit PNG/JP2 icon
    ICP6,
    /// 128x128 32-bit PNG/JP2 icon
    IC07,
    /// 256×256 32-bit PNG/JP2 icon
    IC08,
    /// 512×512 32-bit PNG/JP2 icon
    IC09,

    /// 16x16@2x "retina" 32-bit PNG/JP2 icon
    IC11,
    /// 32x32@2x "retina" 32-bit PNG/JP2 icon
    IC12,
    /// 128x128@2x "retina" 32-bit PNG/JP2 icon
    IC13,
    /// 256x256@2x "retina" 32-bit PNG/JP2 icon
    IC14,
    /// 512x512@2x "retina" 32-bit PNG/JP2 icon
    IC10,
}

impl OsType {
    fn raw(self) -> &'static [u8; 4] {
        match self {
            OsType::IS32 => b"is32",
            OsType::IL32 => b"il32",
            OsType::IH32 => b"ih32",
            OsType::ICP6 => b"icp6",
            OsType::IC07 => b"ic07",
            OsType::IC08 => b"ic08",
            OsType::IC09 => b"ic09",
            // retina
            OsType::IC11 => b"ic11",
            OsType::IC12 => b"ic12",
            OsType::IC13 => b"ic13",
            OsType::IC14 => b"ic14",
            OsType::IC10 => b"ic10",
        }
    }

    pub fn from_size(size: u32, density: u32) -> Option<OsType> {
        match (size, height) {
            ( 16, 1) => Some(OsType::IS32),
            ( 32, 1) => Some(OsType::IL32),
            ( 48, 1) => Some(OsType::IH32),
            ( 64, 1) => Some(OsType::ICP6),
            (128, 1) => Some(OsType::IC07),
            (256, 1) => Some(OsType::IC08),
            (512, 1) => Some(OsType::IC09),

            ( 16, 2) => Some(OsType::IC11),
            ( 32, 2) => Some(OsType::IC12),
            (128, 2) => Some(OsType::IC13),
            (256, 2) => Some(OsType::IC14),
            (512, 2) => Some(OsType::IC10),

            _ => None,
        }
    }
}

pub struct IcnEntry {
    /// The OSType for this entry.
    pub os_type: OSType,
    /// The icon image raw data.
    pub data: Vec<u8>,
}
impl IcnEntry {
    pub fn new(os_type: OSType, data: Vec<u8>) -> IcnEntry {
        IcnEntry { os_type, data, }
    }
}


/// The magic number of an ICNS file
const MAGIC: &'static [u8; 4] = b"icns";

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum OsType {
    /// JPEG 2000 or PNG format (512x512@2x "retina")
    IC10,
    /// JPEG 2000 or PNG format (512x512)
    IC09,
    /// JPEG 2000 or PNG format (256x256@2x "retina")
    IC14,
    /// JPEG 2000 or PNG format (256x256)
    IC08,
    /// JPEG 2000 or PNG format (128x128@2x "retina")
    IC13,
    /// JPEG 2000 or PNG format (128x128)
    IC07,
    /// JPEG 2000 or PNG format (32x32@2x "retina")
    IC12,
    /// JPEG 2000 or PNG format (32x32)
    ICP5,
    /// JPEG 2000 or PNG format (16x16@2x)
    IC11,
    /// JPEG 2000 or PNG format (16x16.png)
    ICP4,
}

impl OsType {
    fn raw(self) -> &'static [u8; 4] {
        match self {
            OsType::IC10 => b"ic10",
            OsType::IC09 => b"ic09",
            OsType::IC14 => b"ic14",
            OsType::IC08 => b"ic08",
            OsType::IC13 => b"ic13",
            OsType::IC07 => b"ic07",
            OsType::IC12 => b"ic12",
            OsType::ICP5 => b"icp5",
            OsType::IC11 => b"ic11",
            OsType::ICP4 => b"icp4",
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
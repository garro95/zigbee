#[derive(Clone, Copy)]
pub enum AddrAndEp{
    None,
    ShortAddressNoEp(u16),
    ShortAddressWithEp(u16, u8),
    LongAddress(u64, u8)
}

pub mod aps;
pub mod framework;
pub mod device_profile;

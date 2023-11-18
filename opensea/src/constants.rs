use ethers::types::Address;
use once_cell::sync::Lazy;

pub const API_BASE_URL: &str = "https://api.opensea.io/api/v2";
pub const SITE_HOST_MAINNET: &str = "https://opensea.io";
pub const SITE_HOST_RINKEBY: &str = "https://rinkeby.opensea.io";

pub const PROTOCOL_STRING: &str = "seaport";

pub static PROTOCOL_ADDRESS: Lazy<Address> = Lazy::new(|| {
    "0x00000000000000adc04c56bf30ac9d3c0aaf14dc"
        .parse()
        .unwrap()
});

pub static PROTOCOL_FEE_RECIPIENT: Lazy<Address> = Lazy::new(|| {
    "0x5b3256965e7c3cf26e11fcaf296dfc8807c01073"
        .parse()
        .unwrap()
});

pub static OPENSEA_ADDRESS: Lazy<Address> = Lazy::new(|| {
    "0x7be8076f4ea4a4ad08075c2508e481d6c946d12b"
        .parse()
        .unwrap()
});

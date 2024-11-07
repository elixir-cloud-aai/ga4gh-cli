pub mod serviceinfo;
pub mod tes;
pub mod wes;

#[derive(Debug)]
pub enum ServiceType {
    TES,
    DRS,
    TRS,
    AAI,
    WES,
}

impl ServiceType {
    pub fn as_str(&self) -> &str {
        match self {
            ServiceType::TES => "TES",
            ServiceType::DRS => "DRS",
            ServiceType::TRS => "TRS",
            ServiceType::AAI => "AAI",
            ServiceType::WES => "WES",
        }
    }
}

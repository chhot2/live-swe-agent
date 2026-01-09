use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a Country.
pub struct Country {
    pub code: String,
    pub name: String,
    pub flag: String,
    pub timezone: String,
    pub language: String,
    pub currency: String,
    pub is_top: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a VirtualIP.
pub struct VirtualIP {
    pub ip: Ipv4Addr,
    pub country_code: String,
    pub country: String,
    pub city: String,
    pub region: String,
    pub timezone: String,
    pub language: String,
    pub currency: String,
    pub isp: String,
    pub proxy_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a IPRange.
pub struct IPRange {
    pub start: Ipv4Addr,
    pub end: Ipv4Addr,
    pub country_code: String,
    pub isp: String,
}

impl IPRange {
    /// Performs contains operation.
    pub fn contains(&self, ip: &Ipv4Addr) -> bool {
        u32::from(*ip) >= u32::from(self.start) && u32::from(*ip) <= u32::from(self.end)
    }
}

/// Represents a CountryDatabase.
pub struct CountryDatabase;

impl CountryDatabase {
    /// Placeholder loaders; replace with real data source.
    pub fn load_all_countries() -> Vec<Country> {
        vec![
            Country {
                code: "US".into(),
                name: "United States".into(),
                flag: "🇺🇸".into(),
                timezone: "America/New_York".into(),
                language: "en-US".into(),
                currency: "USD".into(),
                is_top: true,
            },
            Country {
                code: "GB".into(),
                name: "United Kingdom".into(),
                flag: "🇬🇧".into(),
                timezone: "Europe/London".into(),
                language: "en-GB".into(),
                currency: "GBP".into(),
                is_top: true,
            },
        ]
    }

    /// Loads the top countries.
    pub fn load_top_countries() -> Vec<Country> {
        Self::load_all_countries()
            .into_iter()
            .filter(|c| c.is_top)
            .collect()
    }
}

/// Placeholder loader; replace with real JSON/CSV ingestion.
pub fn load_ip_ranges() -> Vec<IPRange> {
    vec![
        IPRange {
            start: Ipv4Addr::new(8, 8, 4, 0),
            end: Ipv4Addr::new(8, 8, 4, 255),
            country_code: "US".into(),
            isp: "ExampleISP".into(),
        },
        IPRange {
            start: Ipv4Addr::new(1, 0, 0, 0),
            end: Ipv4Addr::new(1, 0, 0, 255),
            country_code: "GB".into(),
            isp: "ExampleISP-GB".into(),
        },
    ]
}

/// Load countries from a JSON file if present; otherwise fallback to placeholder list.
pub fn load_countries_from_file(path: &Path) -> Vec<Country> {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|data| serde_json::from_str::<Vec<Country>>(&data).ok())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(CountryDatabase::load_all_countries)
}

/// Load IP ranges from a JSON file if present; otherwise fallback to placeholder list.
pub fn load_ip_ranges_from_file(path: &Path) -> Vec<IPRange> {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|data| serde_json::from_str::<Vec<IPRange>>(&data).ok())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(load_ip_ranges)
}

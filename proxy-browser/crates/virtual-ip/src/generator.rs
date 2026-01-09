use crate::models::{Country, CountryDatabase, IPRange, VirtualIP};
use anyhow::{anyhow, Result};
use rand::prelude::IteratorRandom;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::net::Ipv4Addr;

#[derive(Clone)]
/// Represents a IPGenerator.
pub struct IPGenerator {
    countries: Vec<Country>,
    ranges: Vec<IPRange>,
}

impl IPGenerator {
    /// Creates a new new.
    pub fn new(countries: Vec<Country>, ranges: Vec<IPRange>) -> Self {
        Self { countries, ranges }
    }

    /// Lists countries.
    pub fn list_countries(&self) -> Vec<Country> {
        self.countries.clone()
    }

    /// Gets the country.
    pub fn get_country(&self, code: &str) -> Option<&Country> {
        self.countries
            .iter()
            .find(|c| c.code.eq_ignore_ascii_case(code))
    }

    /// Generates random.
    pub fn generate_random(&self) -> Result<VirtualIP> {
        let mut rng = thread_rng();
        if self.countries.is_empty() {
            return Err(anyhow!("No countries available"));
        }
        let country = self
            .countries
            .choose(&mut rng)
            .ok_or_else(|| anyhow!("Failed to select random country"))?;
        self.generate_for_country(&country.code)
    }

    /// Generates for country.
    pub fn generate_for_country(&self, code: &str) -> Result<VirtualIP> {
        let country = self
            .get_country(code)
            .ok_or_else(|| anyhow!("Country not found: {}", code))?;

        // Try to pick a range for the country; otherwise random fallback.
        let mut rng = thread_rng();
        let range_opt = self
            .ranges
            .iter()
            .filter(|r| r.country_code.eq_ignore_ascii_case(code))
            .choose(&mut rng);

        let ip = if let Some(range) = range_opt {
            random_ip_in_range(range, &mut rng)
        } else {
            Ipv4Addr::new(rng.gen(), rng.gen(), rng.gen(), rng.gen())
        };

        Ok(VirtualIP {
            ip,
            country_code: country.code.clone(),
            country: country.name.clone(),
            city: "Unknown".into(),
            region: "Unknown".into(),
            timezone: country.timezone.clone(),
            language: country.language.clone(),
            currency: country.currency.clone(),
            isp: range_opt
                .map(|r| r.isp.clone())
                .unwrap_or_else(|| "Unknown ISP".into()),
            proxy_url: None,
        })
    }
}

fn random_ip_in_range(range: &IPRange, rng: &mut impl Rng) -> Ipv4Addr {
    let start = u32::from(range.start);
    let end = u32::from(range.end);
    let span = end.saturating_sub(start).max(1);
    let offset = rng.gen_range(0..=span);
    Ipv4Addr::from(start.saturating_add(offset))
}

/// Convenience to build a demo generator with placeholder data.
pub fn demo_generator() -> IPGenerator {
    let countries = CountryDatabase::load_all_countries();
    let ranges = vec![
        IPRange {
            start: Ipv4Addr::new(8, 8, 8, 0),
            end: Ipv4Addr::new(8, 8, 8, 255),
            country_code: "US".into(),
            isp: "ExampleISP".into(),
        },
        IPRange {
            start: Ipv4Addr::new(1, 1, 1, 0),
            end: Ipv4Addr::new(1, 1, 1, 255),
            country_code: "GB".into(),
            isp: "ExampleISP-GB".into(),
        },
        IPRange {
            start: Ipv4Addr::new(9, 9, 9, 0),
            end: Ipv4Addr::new(9, 9, 9, 255),
            country_code: "DE".into(),
            isp: "ExampleISP-DE".into(),
        },
    ];
    IPGenerator::new(countries, ranges)
}

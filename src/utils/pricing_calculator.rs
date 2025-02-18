use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Constants for price filtering
const MAX_PRICE_ATTOFIL: f64 = 10_000_000_000_000_000_000.0; // 10 FIL per GiB per epoch max
const MIN_PRICE_ATTOFIL: f64 = 1_000.0; // Minimum reasonable price

#[derive(Debug, Deserialize)]
struct FilRepMiner {
    #[serde(default)]
    address: String,
    #[serde(default)]
    price: Value,
    #[serde(rename = "verifiedPrice")]
    #[serde(default)]
    verified_price: Value,
    #[serde(default)]
    region: Option<String>,
    #[serde(default)]
    score: Value,
}

#[derive(Debug, Deserialize)]
struct FilRepResponse {
    miners: Vec<FilRepMiner>,
}

pub struct FilecoinPriceChecker {
    client: reqwest::Client,
}

impl FilecoinPriceChecker {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    fn parse_value_to_f64(v: &Value) -> Option<f64> {
        match v {
            Value::String(s) => s.parse::<f64>().ok(),
            Value::Number(n) => n.as_f64(),
            _ => None,
        }
    }

    fn is_valid_price(price: f64) -> bool {
        price > MIN_PRICE_ATTOFIL && price < MAX_PRICE_ATTOFIL
    }

    pub async fn get_storage_prices(&self) -> Result<Vec<StorageProviderPrice>, reqwest::Error> {
        let response: FilRepResponse = self
            .client
            .get("https://api.filrep.io/api/v1/miners")
            .send()
            .await?
            .json()
            .await?;

        let mut filtered_count = 0;

        let prices = response
            .miners
            .into_iter()
            .filter_map(|m| {
                let price = Self::parse_value_to_f64(&m.price)?;
                let verified_price = Self::parse_value_to_f64(&m.verified_price)?;

                // Filter out invalid prices
                if !Self::is_valid_price(price) || !Self::is_valid_price(verified_price) {
                    filtered_count += 1;
                    return None;
                }

                let score = Self::parse_value_to_f64(&m.score).unwrap_or(0.0);

                Some(StorageProviderPrice {
                    provider: m.address,
                    price_fil: price / 1e18,
                    verified_price_fil: verified_price / 1e18,
                    region: m.region.unwrap_or_else(|| "Unknown".to_string()),
                    reputation_score: score,
                })
            })
            .collect();

        println!("Filtered out {} miners with invalid prices", filtered_count);
        Ok(prices)
    }

    pub fn calculate_storage_cost(
        &self,
        size_gib: f64,
        duration_days: u64,
        prices: &[StorageProviderPrice],
        verified: bool,
    ) -> StorageCostEstimate {
        if prices.is_empty() {
            return StorageCostEstimate::empty(size_gib, duration_days, verified);
        }

        // Filecoin epochs are 30 seconds
        let epochs = (duration_days * 24 * 60 * 60) / 30;

        let price_fn = |p: &StorageProviderPrice| {
            if verified {
                p.verified_price_fil
            } else {
                p.price_fil
            }
        };

        let mut all_prices: Vec<f64> = prices.iter().map(price_fn).collect();
        all_prices.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let median_price = all_prices[all_prices.len() / 2];
        let min_price = all_prices[0];

        StorageCostEstimate {
            size_gib,
            duration_days,
            total_epochs: epochs,
            median_cost_fil: size_gib * median_price * epochs as f64,
            minimum_cost_fil: size_gib * min_price * epochs as f64,
            providers_sampled: prices.len(),
            verified_deal: verified,
        }
    }
}

#[derive(Debug)]
pub struct StorageProviderPrice {
    pub provider: String,
    pub price_fil: f64,
    pub verified_price_fil: f64,
    pub region: String,
    pub reputation_score: f64,
}

#[derive(Debug)]
pub struct StorageCostEstimate {
    pub size_gib: f64,
    pub duration_days: u64,
    pub total_epochs: u64,
    pub median_cost_fil: f64,
    pub minimum_cost_fil: f64,
    pub providers_sampled: usize,
    pub verified_deal: bool,
}

impl StorageCostEstimate {
    fn empty(size_gib: f64, duration_days: u64, verified: bool) -> Self {
        Self {
            size_gib,
            duration_days,
            total_epochs: 0,
            median_cost_fil: 0.0,
            minimum_cost_fil: 0.0,
            providers_sampled: 0,
            verified_deal: verified,
        }
    }
}

impl std::fmt::Display for StorageCostEstimate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nStorage Cost Estimate:\n")?;
        write!(f, "Size: {} GiB\n", self.size_gib)?;
        write!(
            f,
            "Duration: {} days ({} epochs)\n",
            self.duration_days, self.total_epochs
        )?;
        write!(
            f,
            "Deal Type: {}\n",
            if self.verified_deal {
                "Verified"
            } else {
                "Regular"
            }
        )?;
        write!(f, "Median Cost: {:.8} FIL\n", self.median_cost_fil)?;
        write!(f, "Minimum Cost: {:.8} FIL\n", self.minimum_cost_fil)?;
        write!(f, "Providers Sampled: {}\n", self.providers_sampled)?;
        Ok(())
    }
}

#[tokio::test]
async fn main() -> Result<(), reqwest::Error> {
    let checker = FilecoinPriceChecker::new();

    // Get current market prices
    let prices = checker.get_storage_prices().await?;
    println!(
        "Found {} storage providers with valid pricing",
        prices.len()
    );

    // Calculate costs for different scenarios
    let estimates = [
        (1.0, 30, true),     // 1 GiB, 30 days, verified
        (100.0, 180, true),  // 100 GiB, 180 days, verified
        (1.0, 30, false),    // 1 GiB, 30 days, regular
        (100.0, 180, false), // 100 GiB, 180 days, regular
    ];

    for (size, days, verified) in estimates.iter() {
        let estimate = checker.calculate_storage_cost(*size, *days, &prices, *verified);
        println!("{}", estimate);
    }

    Ok(())
}

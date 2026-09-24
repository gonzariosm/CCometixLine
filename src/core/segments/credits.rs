use super::usage::load_usage_data;
use super::{Segment, SegmentData};
use crate::config::{InputData, SegmentId};
use std::collections::HashMap;

/// Extra-usage credits spent this month against the seat's spend limit,
/// e.g. `$23.75/$50 · 48%`. Only rendered when the organization enables credits.
#[derive(Default)]
pub struct CreditsSegment;

impl CreditsSegment {
    pub fn new() -> Self {
        Self
    }

    /// Whether the used percentage is appended (`show_percent` option, default: true)
    fn show_percent_enabled() -> bool {
        crate::config::Config::load()
            .ok()
            .and_then(|config| {
                config
                    .segments
                    .iter()
                    .find(|s| s.id == SegmentId::Credits)
                    .and_then(|sc| sc.options.get("show_percent"))
                    .and_then(|v| v.as_bool())
            })
            .unwrap_or(true)
    }
}

impl Segment for CreditsSegment {
    fn collect(&self, input: &InputData) -> Option<SegmentData> {
        let usage = load_usage_data(input)?;
        let credits = usage.extra_credits?;

        let percent = if credits.limit > 0.0 {
            (credits.used / credits.limit * 100.0).round() as u8
        } else {
            0
        };

        let secondary = if Self::show_percent_enabled() {
            format!("· {}%", percent)
        } else {
            String::new()
        };

        let mut metadata = HashMap::new();
        metadata.insert("credits_used".to_string(), credits.used.to_string());
        metadata.insert("credits_limit".to_string(), credits.limit.to_string());
        metadata.insert("credits_percent".to_string(), percent.to_string());
        metadata.insert("currency".to_string(), credits.currency.clone());

        Some(SegmentData {
            primary: credits.format(),
            secondary,
            metadata,
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::Credits
    }
}

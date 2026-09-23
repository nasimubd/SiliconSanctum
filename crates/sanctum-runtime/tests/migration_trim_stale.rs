use sanctum_runtime::migration::{TrimEvidence,TrimStatus};
use std::time::{Duration,SystemTime};
#[test]fn rejects_stale_evidence(){let now=SystemTime::now();let evidence=TrimEvidence{status:TrimStatus::Observed,observed_at:now-Duration::from_secs(3601)};assert!(!evidence.is_fresh(now));}

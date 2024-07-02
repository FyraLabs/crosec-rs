use std::fmt::Debug;

use bytemuck::{Pod, Zeroable};

#[derive(Debug)]
pub enum EcMkbpEventFingerprintEnrollError {
    LowQuality,
    Immobile,
    LowCoverage,
    Internal,
}

#[derive(Debug)]
pub struct EcMkbpEventFingerprintEnroll {
    pub percentage: u8,
    pub error: Option<EcMkbpEventFingerprintEnrollError>,
}

#[derive(Debug)]
pub struct EcMkbpEventFingerprintMatch {
    pub index: usize,
    /// If `Some`, his means that the fingerprint matched an existing template and the existing template was updated to more accurately match future fingerprints.
    /// `None` if `EC_MKBP_FP_ERR_MATCH_YES`.
    /// `Some(Ok)` if `EC_MKBP_FP_ERR_MATCH_YES_UPDATED`.
    /// `Some(Err)` if `EC_MKBP_FP_ERR_MATCH_YES_UPDATE_FAILED`.
    // TODO: Find the CrOS EC documentation for this and add the link here
    pub update: Option<Result<(), ()>>,
}

#[derive(Debug)]
pub enum EcMkbpEventFingerprintNoMatchError {
    /// `EC_MKBP_FP_ERR_MATCH_NO_INTERNAL` - Probably means there was an internal error.
    Internal,
    /// `EC_MKBP_FP_ERR_MATCH_NO_TEMPLATES` - This either means there are no templates, or something's wrong with the templates. Idk which one.
    Templates,
    /// `EC_MKBP_FP_ERR_MATCH_NO_LOW_QUALITY` - My guess is this might happen if the sensor or finger is dirty
    LowQuality,
    /// `EC_MKBP_FP_ERR_MATCH_NO_LOW_COVERAGE` - My guess is this might happen if only a small part of a finger is sensed
    LowCoverage,
}

#[derive(Debug)]
pub enum EcMkbpEventFingerprintMatchResult {
    Match(EcMkbpEventFingerprintMatch),
    NoMatch(Result<(), EcMkbpEventFingerprintNoMatchError>),
}

#[derive(Debug)]
pub enum EcMkbpEventFingerprintRust {
    /// Contains the enroll progress, as a percentage
    Enroll(EcMkbpEventFingerprintEnroll),
    Match(EcMkbpEventFingerprintMatchResult),
    FingerDown,
    FingerUp,
    ImageReady,
}

const EC_MKBP_EVENT_FINGERPRINT_ERROR_MASK: u32 = 0x0000000F;

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct EcMkbpEventFingerprint {
    fp_events: u32,
}
impl EcMkbpEventFingerprint {
    /// Get a Rust-friendly format. Uses CPU to call and format uses more memory.
    pub fn rust(&self) -> EcMkbpEventFingerprintRust {
        match self.fp_events {
            fp_events if fp_events & (1 << 27) != 0 => {
                EcMkbpEventFingerprintRust::Enroll(EcMkbpEventFingerprintEnroll {
                    percentage: ((self.fp_events & 0x00000FF0) >> 4).try_into().unwrap(),
                    error: match self.fp_events & EC_MKBP_EVENT_FINGERPRINT_ERROR_MASK {
                        0 => None,
                        1 => Some(EcMkbpEventFingerprintEnrollError::LowQuality),
                        2 => Some(EcMkbpEventFingerprintEnrollError::Immobile),
                        3 => Some(EcMkbpEventFingerprintEnrollError::LowCoverage),
                        5 => Some(EcMkbpEventFingerprintEnrollError::Internal),
                        unknown_error => panic!("Unknown error: {unknown_error}"),
                    },
                })
            }
            fp_events if fp_events & (1 << 28) != 0 => EcMkbpEventFingerprintRust::Match({
                let code = self.fp_events & EC_MKBP_EVENT_FINGERPRINT_ERROR_MASK;
                let get_match_index = || ((self.fp_events & 0x0000F000) >> 12) as usize;
                match code {
                    0 => EcMkbpEventFingerprintMatchResult::NoMatch(Ok(())),
                    6 => EcMkbpEventFingerprintMatchResult::NoMatch(Err(
                        EcMkbpEventFingerprintNoMatchError::Internal,
                    )),
                    7 => EcMkbpEventFingerprintMatchResult::NoMatch(Err(
                        EcMkbpEventFingerprintNoMatchError::Templates,
                    )),
                    2 => EcMkbpEventFingerprintMatchResult::NoMatch(Err(
                        EcMkbpEventFingerprintNoMatchError::LowQuality,
                    )),
                    4 => EcMkbpEventFingerprintMatchResult::NoMatch(Err(
                        EcMkbpEventFingerprintNoMatchError::LowCoverage,
                    )),
                    1 => EcMkbpEventFingerprintMatchResult::Match(EcMkbpEventFingerprintMatch {
                        index: get_match_index(),
                        update: None,
                    }),
                    3 => EcMkbpEventFingerprintMatchResult::Match(EcMkbpEventFingerprintMatch {
                        index: get_match_index(),
                        update: Some(Ok(())),
                    }),
                    5 => EcMkbpEventFingerprintMatchResult::Match(EcMkbpEventFingerprintMatch {
                        index: get_match_index(),
                        update: Some(Err(())),
                    }),
                    code => panic!("Unknown error code: {code} ({code:#b})"),
                }
            }),
            fp_events if fp_events & (1 << 29) != 0 => EcMkbpEventFingerprintRust::FingerDown,
            fp_events if fp_events & (1 << 30) != 0 => EcMkbpEventFingerprintRust::FingerUp,
            fp_events if fp_events & (1 << 31) != 0 => EcMkbpEventFingerprintRust::ImageReady,
            fp_events => panic!("Unknown FP event: {fp_events} ({fp_events:#b})"),
        }
    }
}
impl Debug for EcMkbpEventFingerprint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.rust().fmt(f)
    }
}

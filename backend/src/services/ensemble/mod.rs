pub mod averaging;
pub mod voting;
pub mod confidence;

pub use averaging::{
    calculate_weighted_temperature,
    calculate_weighted_average_generic,
};
pub use voting::{
    majority_vote_condition,
    vote_condition,
    majority_vote_with_consensus,
};
pub use confidence::{
    calculate_stddev,
    calculate_cv,
    calculate_confidence,
    calculate_confidence_score,
    get_confidence_details,
    ConfidenceDetails,
};

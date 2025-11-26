pub mod averaging;
pub mod voting;

pub use averaging::{
    calculate_weighted_temperature,
    calculate_weighted_average_generic,
};
pub use voting::{
    majority_vote_condition,
    vote_condition,
    majority_vote_with_consensus,
};

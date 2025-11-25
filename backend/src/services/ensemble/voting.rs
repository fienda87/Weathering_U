use std::collections::HashMap;

/// Perform majority voting on conditions
/// Returns (winning_condition, agreement_ratio)
pub fn majority_vote_condition(conditions: Vec<Option<String>>) -> (String, f32) {
    // Filter None values
    let valid_conditions: Vec<String> = conditions
        .iter()
        .filter_map(|c| c.clone())
        .collect();

    if valid_conditions.is_empty() {
        return ("Unknown".to_string(), 0.0);
    }

    // Count occurrences
    let mut count_map: HashMap<String, usize> = HashMap::new();
    for cond in valid_conditions.iter() {
        *count_map.entry(cond.clone()).or_insert(0) += 1;
    }

    let total_count = valid_conditions.len() as f32;
    
    // Find condition with maximum count
    let (majority_condition, max_count) = count_map
        .iter()
        .max_by_key(|(_, count)| *count)
        .map(|(cond, count)| (cond.clone(), *count as f32))
        .unwrap_or(("Unknown".to_string(), 0.0));

    let agreement_ratio = max_count / total_count;

    (majority_condition, agreement_ratio)
}

/// Simplified voting that returns only the winning condition
pub fn vote_condition(conditions: Vec<Option<String>>) -> String {
    let (condition, _) = majority_vote_condition(conditions);
    condition
}

/// Vote with confidence check (2/3 agreement)
pub fn majority_vote_with_consensus(conditions: Vec<Option<String>>) -> Result<(String, bool), String> {
    let valid_conditions: Vec<String> = conditions
        .iter()
        .filter_map(|c| c.clone())
        .collect();

    if valid_conditions.is_empty() {
        return Err("No conditions to vote on".to_string());
    }

    let (condition, agreement) = majority_vote_condition(conditions);

    // Check for 2/3 consensus (0.667 = 2/3)
    let has_consensus = agreement >= (2.0 / 3.0);

    Ok((condition, has_consensus))
}

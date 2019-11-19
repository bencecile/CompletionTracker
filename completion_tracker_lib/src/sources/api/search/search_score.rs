use std::{
    cmp::{Ordering},
};

pub struct SearchScore {
    score: f64,
    query_len: usize,
    haystack_len: usize,
}
impl SearchScore {
    pub fn new(query_len: usize, haystack_len: usize) -> SearchScore {
        SearchScore {
            score: 0.0,
            query_len,
            haystack_len,
        }
    }
    pub fn score(&self) -> f64 { self.score }

    pub fn update(&mut self, matched_char_count: usize, haystack_index: usize) {
        let haystack_start_index = haystack_index - matched_char_count;

        // The basic score that we reduce based on penalties
        let base_score = matched_char_count as f64;
        // Penalize matches that happen later in the haystack
        // Will be 1 if the match starts right at the beginning
        // Limit down to 0 at the end of the haystack
        let late_match_penalty = 1_f64 - (haystack_start_index as f64) / (self.haystack_len as f64);
        // Penalizes matches that don't get the entire haystack
        let haystack_fragment_penalty = (matched_char_count as f64) / (self.haystack_len as f64);
        // Penalizes matches that don't use up the entire query
        let query_fragment_penalty = (matched_char_count as f64) / (self.query_len as f64);

        let chain_score = base_score * late_match_penalty *
            haystack_fragment_penalty * query_fragment_penalty;
        self.score += chain_score;
    }
}
impl PartialOrd for SearchScore {
    fn partial_cmp(&self, other: &SearchScore) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}
impl PartialEq for SearchScore {
    fn eq(&self, other: &SearchScore) -> bool { self.score.eq(&other.score) }
}

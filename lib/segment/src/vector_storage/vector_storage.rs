use crate::types::{PointOffsetType, ScoreType, VectorElementType, Distance};
use std::cmp::{Ordering, Reverse};
use ordered_float::OrderedFloat;


#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ScoredPointOffset {
    pub idx: PointOffsetType,
    pub score: ScoreType
}

impl Eq for ScoredPointOffset {}

impl Ord for ScoredPointOffset {
    fn cmp(&self, other: &Self) -> Ordering {
        OrderedFloat(self.score).cmp(&OrderedFloat(other.score))
    }
}

impl PartialOrd for ScoredPointOffset {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


/// Trait for vector storage
/// El - type of vector element, expected numerical type
/// Storage operates with internal IDs (PointOffsetType), which always starts with zero and have no skips
pub trait VectorStorage {
    fn vector_dim(&self) -> usize;
    fn vector_count(&self) -> usize;
    fn get_vector(&self, key: PointOffsetType) -> Option<Vec<VectorElementType>>;
    fn put_vector(&mut self, vector: &Vec<VectorElementType>) -> PointOffsetType;
    fn delete(&mut self, key: PointOffsetType);
}

pub trait VectorCounter {
    fn vector_count(&self) -> PointOffsetType;
}

pub trait VectorMatcher {
    fn score_points(
        &self,
        vector: &Vec<VectorElementType>,
        points: &[PointOffsetType],
        top: usize,
        distance: &Distance
    ) -> Vec<ScoredPointOffset>;
    fn score_all(
        &self,
        vector: &Vec<VectorElementType>,
        top: usize,
        distance: &Distance
    ) -> Vec<ScoredPointOffset>;
    fn score_internal(
        &self,
        point: PointOffsetType,
        points: &[PointOffsetType],
        top: usize,
        distance: &Distance
    ) -> Vec<ScoredPointOffset>;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordering() {
        assert!(ScoredPointOffset { idx: 10, score: 0.9} > ScoredPointOffset { idx: 20, score: 0.6})
    }
}
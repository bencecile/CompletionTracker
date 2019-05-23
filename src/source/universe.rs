use std::ops::{Index};

use serde_derive::{Deserialize, Serialize};

use crate::source::{Record, RecordInfo, Relation, SourceItem};

/// A Universe holds a list of series
#[derive(Clone, Deserialize, Serialize)]
pub struct Universe {
    /// The list of series that are in this Universe
    series: Vec< Record<Series> >,
}
impl RecordInfo for Universe {
    fn source_item(&self) -> SourceItem { SourceItem::Universe }
}
impl Universe {
    /// Creates a new empty Universe
    pub fn new() -> Universe {
        Universe { series: Vec::new() }
    }

    /// Gets the highest ID of a series that can be found within this Universe
    pub fn max_series_id(&self) -> u64 { self.series.len() as u64 - 1 }
}
/// Gets the series in a simple manner
impl Index<u64> for Universe {
    type Output = Record<Series>;
    fn index(&self, id: u64) -> &Record<Series> { &self.series[id as usize] }
}

/// A series holds a list of arcs
#[derive(Clone, Deserialize, Serialize)]
pub struct Series {
    /// The ID of the parent Universe
    universe: u64,
    /// The series that are somehow related to this one
    related_series: Vec<Relation>,
    /// The list of arcs that are in this series
    arcs: Vec< Record<Arc> >,
}
impl RecordInfo for Series {
    fn source_item(&self) -> SourceItem { SourceItem::Series(self.universe) }
}
impl Series {
    pub fn max_arcs_id(&self) -> u64 { self.arcs.len() as u64 - 1 }
}
/// Gets the series in a simple manner
impl Index<u64> for Series {
    type Output = Record<Arc>;
    fn index(&self, id: u64) -> &Record<Arc> { &self.arcs[id as usize] }
}

/// There is no sub-category for an Arc
#[derive(Clone, Deserialize, Serialize)]
pub struct Arc {
    /// The ID of the parent universe (of the parent series)
    universe: u64,
    /// The ID of the parent series
    series: u64,
    /// The arcs that are somehow related to this one
    related_arcs: Vec<Relation>,
}
impl RecordInfo for Arc {
    fn source_item(&self) -> SourceItem { SourceItem::Arc(self.universe, self.series) }
}

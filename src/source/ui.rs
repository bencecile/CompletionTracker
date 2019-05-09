use serde_derive::{Serialize};

use crate::source::{Arc, Series, Source, Universe};
use crate::lang::{LangSelect};

/// Creates a vector of universe UI for all of the universes
pub fn create_universes_ui<'u>(universes: &'u Vec<Universe>, sources: &Vec<Source>,
lang: LangSelect) -> Vec< UiUniverse<'u> > {
    universes.iter()
        .map(|universe| UiUniverse::new(universe, sources, lang))
        .collect()
}

/// These are the 
#[derive(Serialize)]
pub struct UiUniverse<'u> {
    name: &'u str,
    source_count: usize,
    series: Vec< UiSeries<'u> >,
}
impl <'u> UiUniverse<'u> {
    pub fn new(universe: &'u Universe, sources: &Vec<Source>, lang: LangSelect)
    -> UiUniverse<'u> {
        let id = universe.id();
        // Try to find this universe in the sources
        let sources_in_universe: Vec<&Source> = sources.iter().filter(|source| {
            if let Some(u_id) = source.universe() {
                *u_id == id
            } else {
                false
            }
        }).collect();
        // Count how many series are in this universe
        let source_count = sources_in_universe.len();
        // Get the stats for each of the series
        let series = universe.series().iter().map(|series| {
            UiSeries::new(series, &sources_in_universe, lang)
        }).collect();

        UiUniverse {
            name: universe.name(lang),
            source_count,
            series,
        }
    }
}
#[derive(Serialize)]
struct UiSeries<'s> {
    name: &'s str,
    source_count: usize,
    arcs: Vec< UiArc<'s> >,
}
impl <'s> UiSeries<'s> {
    /// Gets the stats from a series using a vector of sources that are in this universe
    fn new(series: &'s Series, sources_in_universe: &Vec<&Source>, lang: LangSelect)
    -> UiSeries<'s> {
        let id = series.id();
        // Find the sources that belong to this series
        let sources_in_series: Vec<&Source> = sources_in_universe.iter().filter(|source| {
            if let Some(s_id) = source.series() {
                *s_id == id
            } else {
                false
            }
        }).map(|s| *s).collect();
        // Count up how many we got
        let source_count = sources_in_series.len();
        // Create the stats for each arc
        let arcs = series.arcs().iter().map(|arc| {
            UiArc::new(arc, &sources_in_series, lang)
        }).collect();

        UiSeries {
            name: series.name(lang),
            source_count,
            arcs,
        }
    }
}
#[derive(Serialize)]
struct UiArc<'a> {
    name: &'a str,
    source_count: usize,
}
impl <'a> UiArc<'a> {
    fn new(arc: &'a Arc, sources_in_series: &Vec<&Source>, lang: LangSelect) -> UiArc<'a> {
        let id = arc.id();
        // Just get the count directly
        let source_count = sources_in_series.iter().fold(0, |count, source| {
            if let Some(a_id) = source.arc() {
                if *a_id == id {
                    count + 1
                } else {
                    count
                }
            } else {
                count
            }
        });

        UiArc {
            name: arc.name(lang),
            source_count,
        }
    }
}

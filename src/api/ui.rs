// use serde_derive::{Serialize};

// use crate::source::{
//     RelatedLink,
//     Source,
//     Universe, Series, Arc,
// };

// /// This is the model of a record for the UI
// #[derive(Clone, Serialize)]
// pub struct RecordUi<'r, T> {
//     name: &'r str,
//     aliases: &'r [String],
//     description: &'r str,
//     image_path: Option<String>,
//     related_links: Vec<RelatedLinkUi<'r>>,
//     record_info: T,
// }
// impl <'r, T> RecordUi<'r, T> {
//     // fn new<R>(record: &'r Record<R>, info_ui: T, lang: Lang)
//     // -> Result<RecordUi<T, 'r>, SourceError> {
//     //     // Get the aliases by looking up the langauge
//     //     // If we can't find any, just use an empty vector
//     //     let aliases = record.aliases.get(&lang)
//     //         .unwrap_or_else(|| Vec::new())
//     //         .map(|alias| alias.as_str())
//     //         .collect();
//     //     let image_path = if let Some(image_result) = record.image_path() {
//     //         image_result?.
//     //     };

//     //     RecordUi {
//     //         name: record.name.get_str(lang),
//     //         aliases,
//     //         description: record.description.get_str(lang),
//     //     }
//     // }
// }


// /// The UI model for a RelatedLink
// #[derive(Copy, Clone, Serialize)]
// pub struct RelatedLinkUi<'r> {
//     url: &'r str,
//     description: &'r str,
// }
// impl <'r> RelatedLinkUi<'r> {
//     fn new(related_link: &'r RelatedLink) -> RelatedLinkUi<'r> {
//         RelatedLinkUi {
//             url: related_link.url(),
//             description: related_link.description(lang),
//         }
//     }
// }

// /// Creates a vector of universe UI for all of the universes
// pub fn create_universes_ui<'u>(universes: &'u Vec<Universe>, sources: &Vec<Source>,
// lang: Lang) -> Vec< UniverseUi<'u> > {
//     universes.iter()
//         .map(|universe| UniverseUi::new(universe, sources, lang))
//         .collect()
// }

// /// This is the information the UI needs to be able to show universes
// #[derive(Serialize)]
// pub struct UniverseUi<'u> {
//     name: &'u str,
//     id: u64,
//     source_count: usize,
//     series: Vec< SeriesUi<'u> >,
// }
// impl <'u> UniverseUi<'u> {
//     pub fn new(universe: &'u Universe, sources: &Vec<Source>, lang: Lang)
//     -> UniverseUi<'u> {
//         let id = universe.id;
//         // Try to find this universe in the sources
//         let sources_in_universe: Vec<&Source> = sources.iter().filter(|source| {
//             source.universe.map_or(false, |u_id| u_id == id)
//         }).collect();
//         // Count how many series are in this universe
//         let source_count = sources_in_universe.len();
//         // Get the stats for each of the series
//         let series = universe.series.iter().map(|series| {
//             SeriesUi::new(series, &sources_in_universe, lang)
//         }).collect();

//         UniverseUi {
//             name: universe.name(lang),
//             id,
//             source_count,
//             series,
//         }
//     }
// }
// #[derive(Serialize)]
// struct SeriesUi<'s> {
//     name: &'s str,
//     id: u64,
//     source_count: usize,
//     arcs: Vec< ArcUi<'s> >,
// }
// impl <'s> SeriesUi<'s> {
//     /// Gets the stats from a series using a vector of sources that are in this universe
//     fn new(series: &'s Series, sources_in_universe: &Vec<&Source>, lang: Lang)
//     -> SeriesUi<'s> {
//         let id = series.id;
//         // Find the sources that belong to this series
//         let sources_in_series: Vec<&Source> = sources_in_universe.iter().filter(|source| {
//             source.series.map_or(false, |s_id| s_id == id)
//         }).map(|s| *s).collect();
//         // Count up how many we got
//         let source_count = sources_in_series.len();
//         // Create the stats for each arc
//         let arcs = series.arcs.iter().map(|arc| {
//             ArcUi::new(arc, &sources_in_series, lang)
//         }).collect();

//         SeriesUi {
//             name: series.name(lang),
//             id,
//             source_count,
//             arcs,
//         }
//     }
// }
// #[derive(Serialize)]
// struct ArcUi<'a> {
//     name: &'a str,
//     id: u64,
//     source_count: usize,
// }
// impl <'a> ArcUi<'a> {
//     fn new(arc: &'a Arc, sources_in_series: &Vec<&Source>, lang: Lang) -> ArcUi<'a> {
//         let id = arc.id;
//         // Just get the count directly
//         let source_count = sources_in_series.iter().fold(0, |count, source| {
//             source.arc.map_or(count, |a_id| if a_id == id {
//                 count + 1
//             } else {
//                 count
//             })
//         });

//         ArcUi {
//             name: arc.name(lang),
//             id,
//             source_count,
//         }
//     }
// }

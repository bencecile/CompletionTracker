use rouille::{Response, router};

use super::{ApiEndpoint, Create};
use super::data::{self, DataError, DataResult, EmptySubData, RecordData};
use crate::api::{Api, RequestPipeline};

pub struct UniverseEndpoint;
impl UniverseEndpoint {
    // /// Gets the universe with the given ID.
    // fn get<'a>(api: &'a Api, id: u64) -> Result<RecordUI<UniverseUI, 'a>, SourceError> {

    // }

    /// Creates a universe from the data
    fn create(api: &Api, mut data: RecordData<EmptySubData>) -> DataResult<u64> {
        // Ignore the ID since we don't need one for creation
        // We always need a name
        let names = data.convert_names()?;

        // Validate everything else before we start adding things to the DB
        let image = if let Some(image) = data.convert_image() { Some(image?) } else { None };
        let related_links = data.create_related_links()?;

        // Get the source database
        let mut source_db = api.lock_db();

        let u_id = source_db.add_universe_tag(names);
        source_db.modify_universe_tag(u_id, |mut universe| {
            // Add the description if we have one
            if let Some(descriptions) = data.descriptions {
                universe.descriptions = descriptions;
            }
            // Set the image if we have one
            if let Some(image) = image {
                image.save_and_record(&mut universe)?;
            }
            // Set the links (we could also just not have any)
            universe.related_links = related_links;

            Ok(universe)
        }).map_err(|e| DataError::SourceError(e))?;

        source_db.save_universe_tags()?;

        // Return the Universe ID so that it can be found again
        Ok(u_id)
    }
}
impl ApiEndpoint for UniverseEndpoint {
    const NAME: &'static str = "Universe";
    
    fn match_request<'a>(pipeline: &RequestPipeline<'a>) -> Option<Response> {
        router!(pipeline.req,
            // (GET) (/api/universe/{id: u64}) => {
            //     Some(UniverseEndpoint::convert_endpoint_response(
            //         "GET", pipeline.settings.lang(),
            //         || UniverseEndpoint::get(pipeline.api, id)
            //     ))
            // },
            (POST) (/api/universe/create) => {
                // Try and create the data for the request
                Some(match data::from_request(pipeline.req) {
                    Ok(data) => UniverseEndpoint::convert_endpoint_response::<Create, _, _>(
                        UniverseEndpoint::create(pipeline.api, data)
                    ),
                    Err(err) => UniverseEndpoint::convert_endpoint_failure::<Create, _>(err),
                })
            },
            _ => { None },
        )
    }
}

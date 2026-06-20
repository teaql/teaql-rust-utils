use crate::macros::*;

use teaql_tool_extra::geo::GeoTool;

define_context_facade!("extra", geo, ContextGeoExt, ContextGeoFacade);

#[cfg(feature = "extra")]
impl<'a> ContextGeoFacade<'a> {
    delegate_comment! { GeoTool::new(),
        fn distance_km(&self, lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64
    }
}

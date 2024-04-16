use crate::{configuration, routes, telemetry};
use pavex::blueprint::Blueprint;
use pavex::kit::ApiKit;
use pavex::f;

/// The main blueprint, containing all the routes, middlewares, constructors and error handlers
/// required by our API.
pub fn blueprint() -> Blueprint {
    let mut bp = Blueprint::new();
    ApiKit::new().register(&mut bp);
    telemetry::register(&mut bp);
    configuration::register(&mut bp);

    routes::register(&mut bp);
    bp.singleton(f!(crate::routes::tts::TtsContext::new));
    bp
}

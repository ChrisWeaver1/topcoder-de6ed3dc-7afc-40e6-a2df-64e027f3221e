
use rocket::{Request, Response, catch};
use rocket::response::{Result};
use rocket::http::{Header, Status};

#[catch(401)]
pub fn catch_401<'a>(_req: &Request) -> Result<'a> {
    // build a respose
    return Response::build()
        // add header
        .header(Header::new("WWW-Authenticate", "Basic realm=\"topcoder-challenge\""))
        // set status code
        .status(Status::Unauthorized)
        // wrap response as result
        .ok();
}

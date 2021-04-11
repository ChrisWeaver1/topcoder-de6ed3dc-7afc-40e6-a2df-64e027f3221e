use rocket::{
    Request,
    Outcome,
    outcome::IntoOutcome,
    http::Status,
    request::FromRequest,
    self
};
extern crate base64;

pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

// implement FromRequest from rocket on our BasicAuth struct
// 'a and 'r are lifetime types, trying not to worry about these at the moment because it seems like a giant 
// rabbit hole, but they seem like generic passthrough values
impl<'a, 'r> FromRequest<'a, 'r> for BasicAuth {
    type Error = ();

    fn from_request(request: &Request) -> Outcome<Self, (Status, <Self as FromRequest<'a, 'r>>::Error), ()> {
        
        // get Authorization header value
        // should look something like "basic owod4ioe66w=="
        let auth_header = request.headers().get_one("Authorization");

        // Unwrap & check option value
        if let Some(auth_header) = auth_header {

            // split on whitespace, collect into array
            // type was originally Vec<_>, look into what that means, 
            // I imagine its just a way of taking the value of split_whitespace
            let split = auth_header.split_whitespace().collect::<Vec<&str>>();
    
            // check length of split value is 2
            if split.len() != 2 {
                return Outcome::Failure((Status::Unauthorized, ()));
            }
    
            // set values as variables
            let (basic, payload) = (split[0], split[1]);
            // check header uses basic auth
            if basic != "Basic" {
                return Outcome::Failure((Status::Unauthorized, ()));
            }
    
            // decode base64 value, seems to decode into byte array
            let decoded = base64::decode(payload)
                .ok()
                .into_outcome((Status::Unauthorized, ()))?;
    
            // create a string from decoded array
            let decoded_str = String::from_utf8(decoded)
                .ok()
                .into_outcome((Status::Unauthorized, ()))?;
    
            // split username:password
            let split = decoded_str.split(":").collect::<Vec<_>>();
    
            // check array value is expected length
            if split.len() != 2 {
                return Outcome::Failure((Status::Unauthorized, ()));
            }
            
            // set username and password variables
            let (username, password) = (split[0].to_string(), split[1].to_string());
    
            // check username, password values
            // return success if correct, unauthorized otherwise 
            if username == "topcoder" && password == "rocks" {
                return Outcome::Success(BasicAuth {
                    username,
                    password
                })
            }
            else {
                return Outcome::Failure((Status::Unauthorized, ()));
            }
        }
        else {
            // No authorization header value
            return Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}
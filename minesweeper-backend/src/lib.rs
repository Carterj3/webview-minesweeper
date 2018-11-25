// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate web_view;

extern crate chrono;

extern crate rand;

pub mod engine;
pub mod common;

pub mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
     error_chain!{
        foreign_links {
            /* NoneError doesn't like to be implemented. Just use `.ok_or("Nothing")?` instead of only `?` */
            // Nothing(::std::option::NoneError);
        }
    }
}
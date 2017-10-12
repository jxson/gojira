use std;
use hyper;
use native_tls;

error_chain!{
    foreign_links {
        IOError(std::io::Error);
        UriError(hyper::error::UriError);
        TLSError(native_tls::Error);
    }
}

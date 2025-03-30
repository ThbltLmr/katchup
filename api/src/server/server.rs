struct HttpSuccessCode;

impl HttpSuccessCode {
    const OK: u16 = 200;
}

struct HttpErrorCode;

impl HttpErrorCode {
    const BAD_REQUEST: u16 = 400;
    const NOT_FOUND: u16 = 404;
    const INTERNAL_SERVER_ERROR: u16 = 500;
    const SERVICE_UNAVAILABLE: u16 = 503;
}

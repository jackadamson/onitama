use actix_web::http::header;
use actix_web::HttpRequest;

pub fn get_identifier(req: &HttpRequest) -> String {
    let user_agent = match req
        .headers()
        .get(header::USER_AGENT)
        .map(|value| value.to_str())
    {
        Some(Ok(value)) => value,
        Some(Err(_)) => "Unknown Browser",
        None => "Unknown Browser",
    };
    let info = req.connection_info();
    let ip = match info.realip_remote_addr() {
        None => "unknown",
        Some(ip) => ip,
    };
    let identifier = format!("{} ({})", ip, user_agent);
    return identifier;
}

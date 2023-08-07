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

pub fn get_useragent(req: &HttpRequest) -> String {
    return match req
        .headers()
        .get(header::USER_AGENT)
        .map(|value| value.to_str())
    {
        Some(Ok(value)) => value,
        Some(Err(_)) => "Unknown Browser",
        None => "Unknown Browser",
    }.to_string();
}

pub fn get_ip(req: &HttpRequest) -> String {
    let info = req.connection_info();
    return match info.realip_remote_addr() {
        None => "unknown".to_string(),
        Some(ip) => ip.to_string(),
    };
}
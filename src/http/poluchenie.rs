use ohkami::{Response, Status};

pub async fn add_poluchenie() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("added poluchenie");
    response
}

pub async fn get_poluchenie() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("got poluchenie");
    response
}

pub async fn update_poluchenie() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("updated poluchenie");
    response
}

pub async fn remove_poluchenie() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("removed poluchenie");
    response
}

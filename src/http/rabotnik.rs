use ohkami::{Response, Status};

pub async fn add_rabotnik() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("added rabotnik");
    response
}

pub async fn get_rabotnik() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("got rabotnik");
    response
}

pub async fn update_rabotnik() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("updated rabotnik");
    response
}

pub async fn remove_rabotnik() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("removed rabotnik");
    response
}

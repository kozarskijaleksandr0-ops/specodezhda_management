use ohkami::{Response, Status};

pub async fn add_specodezhda() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("added specodezhda");
    response
}

pub async fn get_specodezhda() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("got specodezhda");
    response
}

pub async fn update_specodezhda() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("updated specodezhda");
    response
}

pub async fn remove_specodezhda() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("removed specodezhda");
    response
}

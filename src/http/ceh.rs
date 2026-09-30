use ohkami::{Response, Status};

pub async fn add_ceh() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("added ceh");
    response
}

pub async fn get_ceh() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("got ceh");
    response
}

pub async fn update_ceh() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("updated ceh");
    response
}

pub async fn remove_ceh() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("removed ceh");
    response
}

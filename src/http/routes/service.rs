use std::ops::Sub;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use serde::Serialize;
use crate::http::model::response::HttpResponse;
use crate::State;

#[derive(Serialize)]
struct ServiceStatus {
    core: bool,
    controller: bool,
}

pub fn status(state: Arc<Mutex<State>>) -> Option<HttpResponse> {
    let controller_time = { state.lock().unwrap().controller_ping };
    let mut status = ServiceStatus { core: true, controller: false };

    if let Some(controller_time) = controller_time {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let difference = current_time.sub(controller_time).as_secs();
        status.controller = difference < 5;
    }

    Some(HttpResponse::new().body(serde_json::to_string(&status).unwrap().as_str()))
}
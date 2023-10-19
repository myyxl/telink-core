use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use crate::http::model::response::HttpResponse;
use crate::State;

pub fn status(state: Arc<Mutex<State>>) -> Option<HttpResponse> {
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

    if let None = state.lock().unwrap().controller_last_ping {
        return Some(
            HttpResponse::new()
                .body("{\"core\": true, \"controller\": false}")
        )
    }

    let controller_time = state.lock().unwrap().controller_last_ping.unwrap().as_millis();
    
    let difference = current_time - controller_time;
    let time = Duration::from_millis(difference.try_into().unwrap()).as_secs();

    if time > 5 {
        Some(
            HttpResponse::new()
                .body("{\"core\": true, \"controller\": false}")
        )
    } else {
        Some(
            HttpResponse::new()
                .body("{\"core\": true, \"controller\": true}")
        )
    }

    

    
}
use nickel::{Nickel, HttpRouter};
use nickel::status::StatusCode;

use librespot::spirc::SpircManager;

use rustc_serialize::json::ToJson;

fn ok<T>(_: T) -> StatusCode {
    StatusCode::Ok
}

pub fn run(spirc: SpircManager) {
    let mut server = Nickel::new();

    let spirc_device_list = spirc.clone();
    server.get("/devices",
               middleware!(spirc_device_list.devices().to_json()));

    let spirc_cmd = spirc.clone();
    server.put("/:device/:cmd",
               middleware!(|req, res| {
        match req.param("cmd") {
            Some("pause") => ok(spirc_cmd.send_pause(req.param("device").unwrap())),
            Some("play") => ok(spirc_cmd.send_play(req.param("device").unwrap())),
            Some("next") => ok(spirc_cmd.send_next(req.param("device").unwrap())),
            Some("prev") => ok(spirc_cmd.send_prev(req.param("device").unwrap())),
            _ => StatusCode::NotFound,
        }
    }));

    server.listen("127.0.0.1:6767");
}

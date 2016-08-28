#[macro_use]
extern crate nickel;
extern crate librespot;
extern crate getopts;
extern crate rustc_serialize;

#[macro_use]
extern crate log;

use std::process::exit;
use std::thread;

use librespot::spirc::SpircManager;
use librespot::main_helper;

mod web_server;

fn usage(program: &str, opts: &getopts::Options) -> String {
    let brief = format!("Usage: {} [options]", program);
    format!("{}", opts.usage(&brief))
}

fn main() {
    let mut opts = getopts::Options::new();
    main_helper::add_session_arguments(&mut opts);
    main_helper::add_authentication_arguments(&mut opts);
    main_helper::add_player_arguments(&mut opts);

    let args: Vec<String> = std::env::args().collect();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            error!("Error: {}\n{}", f.to_string(), usage(&args[0], &opts));
            exit(1)
        }
    };

    let session = main_helper::create_session(&matches);
    let credentials = main_helper::get_credentials(&session, &matches);
    session.login(credentials).unwrap();

    let spirc_runner = SpircManager::new(session.clone(), None);
    let spirc_remote = spirc_runner.clone();
    thread::spawn(move || spirc_runner.run());
    thread::spawn(move || {
        web_server::run(spirc_remote);
    });

    loop {
        session.poll();
    }
}

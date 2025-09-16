use std::sync::Mutex;

const HISTORY_LIMIT: usize = 300;

pub struct Log {
    history: Vec<String>,
}

pub static LOG: Mutex<Log> = Mutex::new(Log {
    history: Vec::new(),
});

pub fn add_to_log(text: String) {
    LOG.lock().unwrap().history.push(text);
}

pub fn actor_attack(attacker: &String, defender: &String) {
    LOG.lock()
        .unwrap()
        .history
        .push(format!("{attacker} attacks {defender}."));
}

pub fn actor_pos_update(actor: &String, position: &(i16, i16)) {
    LOG.lock()
        .unwrap()
        .history
        .push(format!("{actor} moved to {}/{}", position.0, position.1));
}

pub fn tail() -> Vec<String> {
    let mut tail_txt: Vec<String> = Vec::with_capacity(10);
    for n in 0..10 {
        if let Some(x) = LOG.lock().unwrap().history.get(n) {
            println!("{}", x);
            tail_txt.push(String::from(x));
        }
    }

    tail_txt
}

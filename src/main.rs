static WM: &str = "\n\n[ JET Profile Archiver v0.1.0 ] by kiobu\n\n";

extern crate chrono;

use std::io::ErrorKind;
use chrono::Local;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::{thread, time};

fn setup() -> String {
    let now = Local::now().to_rfc3339().replace(":", "_");
    println!("Created backup archive with timestamp {:?}", now);
    match fs::create_dir("./backups") {
        Ok(_) => (),
        Err(err) => {
            if err.kind() == ErrorKind::AlreadyExists {
                ();
            } else {
                panic!(err);
            }
        }
    };
    let dir = format!("./backups/{}", now);
    fs::create_dir(dir).unwrap();
    return now;
}

struct Profile {
    aid: String,
    dir: PathBuf, // Dir path
    conts: Vec<PathBuf>
}

impl Profile {
    fn create(profile: PathBuf) -> Profile {
        let directory = profile.clone();
        let aid = match profile.file_name() {
            Some(aid) => match aid.to_str() {
                Some(aid) => String::from(aid),
                _ => panic!("None.")
            },
            _ => String::from("Fail")
        };
        let mut contents: Vec<PathBuf> = Vec::new();
        for files in fs::read_dir(profile).unwrap() {
            match files {
                Ok(file) => {
                    contents.push(PathBuf::from(file.path()))
                },
                Err(_) => ()
            };
        }
        return Profile {
            aid: aid,
            dir: directory,
            conts: contents
        }
    }

    fn archive(self, timestamp: &String) {
        let destination: String = format!("./backups/{}/{}", timestamp, self.aid);
        fs::create_dir(&destination).unwrap();

        for filepath in self.conts {
            let destfile = format!("{}/{}", destination, filepath.file_name().unwrap().to_os_string().into_string().unwrap());
            fs::copy(filepath, destfile).unwrap();
        }
        println!("{} has been archived.", self.aid);
    }
}

fn main() {
    println!("{}", &WM);
    let timestamp = setup();

    match fs::canonicalize("./user/profiles") {
        Ok(path) => {
            let ids = fs::read_dir(path).unwrap();

            let mut profiles: Vec<Profile> = Vec::new();
        
            for dir in ids {
                match dir {
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_dir() {
                            profiles.push(Profile::create(path));
                        }
                    },
                    Err(_) => ()
                };
            }
        
            for p in profiles { p.archive(&timestamp) };
        },
        Err(err) => { println!("Could not find user/profiles ... is this in the same directory as Server.exe?\n---> {}", err); return; }
    };
    println!("Launching Server ...");
    thread::sleep(time::Duration::from_secs(2)); // Allow user to see messages before spawning process on same thread.
    Command::new("Server.exe").spawn().expect("Failed to execute Server.exe.");
}

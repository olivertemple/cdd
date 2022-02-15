use std::env;
use std::fs;
use std::io::prelude::*;

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut paths = read_paths();

    if args[1] == "-a" {
        let present = paths.iter().any(|s| s.0 == args[2]);
        if !present {
            let new_path = (args[2].to_string(), args[3].to_string());
            paths.push(new_path);
        } else {
            let index = paths.iter().position(|s| s.0 == args[2]).unwrap();
            paths[index].1 = args[3].to_string();
        }
        write_paths(paths);
        println!("Successfully added path {} to name {}", args[3], args[2]);
    }else if args[1] == "-d" {
        let present = paths.iter().any(|s| s.0 == args[2]);
        if present {
            let index = paths.iter().position(|s| s.0 == args[2]).unwrap();
            paths.remove(index);
        }   
        write_paths(paths);
        println!("Successfully removed {}", args[2]);
    } else if args[1] == "-l"{
        for path in paths {
            println!("{}: {}", path.0, path.1);
        }
    } else {
        let present = paths.iter().any(|s| s.0 == args[1]);
        if present {
            let index = paths.iter().position(|s| s.0 == args[1]).unwrap();
            let path = &paths[index].1;
            println!("{}", path);
        }
    }
}

fn write_paths(paths: Vec<(String, String)>){
    let app_data_path = env::var("APPDATA").unwrap();
    let file_path = app_data_path + "\\cdd\\data.txt";
    let data_to_write = paths.into_iter().fold(String::new(), |acc, s| acc + &s.0 + "," + &s.1 + "\n");
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_path)
        .unwrap();
    file.write(data_to_write.trim().as_bytes()).unwrap();
}
fn read_paths() -> Vec<(String, String)> {
    let app_data_path = env::var("APPDATA").unwrap();
    let path = app_data_path + "\\cdd\\data.txt";
    let file_path = std::path::Path::new(&path);
    if !file_path.exists(){
        fs::create_dir_all(file_path.parent().unwrap()).unwrap();
    }
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents = contents.trim().to_string();
    if file.metadata().unwrap().len() == 0 {
        return Vec::new();
    }
    let items: Vec<(String, String)> = contents.split("\n").map(|s| {
        let (a, b) = s.trim().split_at(s.find(',').unwrap());
        (a.to_string(), b.trim_start_matches(',').to_string())
    }).collect();
    items
}

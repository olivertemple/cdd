use std::env;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut paths = read_paths();

    if args[1] == "-a" {
        let present = paths.iter().any(|s| s.0 == args[2].to_lowercase());
        if !present {
            let new_path = (args[2].to_string().to_lowercase(), args[3].to_string());
            paths.push(new_path);
        } else {
            let index = paths.iter().position(|s| s.0 == args[2].to_lowercase()).unwrap();
            paths[index].1 = args[3].to_string();
        }
        write_paths(paths);
        println!("Successfully added path {} to name {}", args[3], args[2]);
    }else if args[1] == "-d" {
        let present = paths.iter().any(|s| s.0 == args[2].to_lowercase());
        if present {
            let index = paths.iter().position(|s| s.0 == args[2].to_lowercase()).unwrap();
            paths.remove(index);
        }   
        write_paths(paths);
        println!("Successfully removed {}", args[2]);
    } else if args[1] == "-l"{
        for path in paths {
            println!("{}: {}", path.0, path.1);
        }
    } else {
        let splitter = args[1].splitn(2, "/").collect::<Vec<&str>>();
        let root = splitter[0].to_lowercase();
        let mut additional_path = "";
        if splitter.len() > 1 {
            additional_path = splitter[1];
        }
        let present = paths.iter().any(|s| s.0 == root);
        if present {
            let index = paths.iter().position(|s| s.0 == root).unwrap();
            let path = &paths[index].1;
            let mut full_path = PathBuf::from(path);
            full_path.push(additional_path);
            println!("{:?}", full_path);
        }else{
            println!("{}", root);
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

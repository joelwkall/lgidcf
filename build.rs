use std::env;
use std::fs;
use std::path::Path;

fn main() {

    let out_dir = env::var("OUT_DIR").unwrap();
	
	let resources_path = env::var("CARGO_MANIFEST_DIR").unwrap() +"/resources";
	let settings_path = env::var("CARGO_MANIFEST_DIR").unwrap() +"/settings";
	let devices_path = env::var("CARGO_MANIFEST_DIR").unwrap() +"/devices";
	
    let dest_path_base = Path::new(&out_dir).parent().unwrap().parent().unwrap().parent().unwrap().to_str().unwrap();
	
	let dest_resources_path = format!("{}/{}",dest_path_base,  "resources");
	fs::create_dir_all(&dest_resources_path).unwrap();
	
	let dest_settings_path = format!("{}/{}",dest_path_base,  "settings");
	fs::create_dir_all(&dest_settings_path).unwrap();
	
	let dest_devices_path = format!("{}/{}",dest_path_base,  "devices");
	fs::create_dir_all(&dest_devices_path).unwrap();
	
	let files = fs::read_dir(&resources_path).unwrap();
	for f in files {
	
		println!("found file");
		
		let p = f.unwrap().path();
		let file = Path::new(&p);
		
		if file.is_file() {
		
			let name = file.file_name().unwrap().to_str().unwrap();
			
			let source_path = format!("{}/{}", resources_path, name);
			let dest_path = format!("{}/{}",dest_resources_path,  name);
			
			println!("source: {}",source_path);
			println!("desc: {}",dest_path);
			
			fs::copy(source_path, dest_path).unwrap();
		
		}
	}
	
	let files = fs::read_dir(&settings_path).unwrap();
	for f in files {
	
		println!("found file");
		
		let p = f.unwrap().path();
		let file = Path::new(&p);
		
		if file.is_file() {
		
			let name = file.file_name().unwrap().to_str().unwrap();
			
			let source_path = format!("{}/{}", settings_path, name);
			let dest_path = format!("{}/{}",dest_settings_path,  name);
			
			println!("source: {}",source_path);
			println!("desc: {}",dest_path);
			
			fs::copy(source_path, dest_path).unwrap();
		
		}
	}
	
	let files = fs::read_dir(&devices_path).unwrap();
	for f in files {
	
		println!("found file");
		
		let p = f.unwrap().path();
		let file = Path::new(&p);
		
		if file.is_file() {
		
			let name = file.file_name().unwrap().to_str().unwrap();
			
			let source_path = format!("{}/{}", devices_path, name);
			let dest_path = format!("{}/{}",dest_devices_path,  name);
			
			println!("source: {}",source_path);
			println!("desc: {}",dest_path);
			
			fs::copy(source_path, dest_path).unwrap();
		
		}
	}
}
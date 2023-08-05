use std::fs;
use std::io;


// fn open_dir(dir: &String) -> File {
// 	let file_handek = match File::open(dir) {
// 		Ok(file) => file,
// 		Err(err) => {
// 			panic!("Error: Can't open current directory {dir}: {:?}", err);
// 		},
// 	};
// 	return file_handek;
// }

pub fn list_files_in_dir(current_dir: &String) -> io::Result<Vec<String>> {

	println!("current working dir: {}", current_dir);

	let entries = fs::read_dir(current_dir)?;
	let file_names: Vec<String> = entries
		.filter_map(|entry| {
		let path = entry.ok()?.path();
		if path.is_file() {
			// if path.file_name()?.to_str().map(|s| s.to_owned().starts_with('.')) {
				path.file_name()?.to_str().map(|s| s.to_owned())
			// }
		} else {
			None
		}
		}).collect();
	Ok(file_names)


}

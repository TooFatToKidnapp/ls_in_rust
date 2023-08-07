use std::fs;
use std::io;

#[derive(Debug)]
pub enum FileType {
	Dir,
	File,
	Symlink,
	Other,
}

#[derive(Debug)]
pub struct FileInDir {
	name: String,
	file_type: FileType,
}


// fn open_dir(dir: &String) -> File {
// 	let file_handek = match File::open(dir) {
// 		Ok(file) => file,
// 		Err(err) => {
// 			panic!("Error: Can't open current directory {dir}: {:?}", err);
// 		},
// 	};
// 	return file_handek;
// }

pub fn get_files_in_dir(current_dir: &String) -> io::Result<Vec<FileInDir>> {

	println!("current working dir: {}", current_dir);

	let entries = fs::read_dir(current_dir)?;
	let file_names: Vec<FileInDir> = entries
		.filter_map(|entry: Result<fs::DirEntry, io::Error>| {
		let mut file = FileInDir {
			name: String::new(),
			file_type: FileType::Other,
		};
		let path: std::path::PathBuf = entry.ok()?.path();
		let meta_data = path.metadata().ok()?;
		if meta_data.is_symlink() { // dug: not functional
			file.file_type = FileType::File;
		}
		else if meta_data.is_dir() {
			file.file_type = FileType::Dir;
		} else if meta_data.is_file() {
			file.file_type = FileType::Symlink;
		}
		file.name = path.file_name()?.to_str()?.to_owned();
		if file.name.starts_with('.') == false {
			Some(file)
		} else {
			None
		}
		}).collect();
	Ok(file_names)

}

use std::env;
mod list_files_in_dir;
use list_files_in_dir::list_files_in_dir;
fn main() {

    let argv: Vec<String> = env::args().collect();
    let argc: usize = argv.len();
    let current_working_dir =  match env::current_dir() {
        Ok(dir) => dir.into_os_string().into_string().unwrap(),
        Err(err) => {
            panic!("Problem getting the current working dir: {:?}", err);
        }
    };
    match argc {
        1 => match list_files_in_dir(&current_working_dir) {
            Ok(files) => println!("{:?}", files),
            Err(err) => panic!("error getting files {:?}" , err)
        },
        _ => todo!(),
    }

}

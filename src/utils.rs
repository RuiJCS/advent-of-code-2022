pub mod utils {
    use std::fs::File;
    use std::io::prelude::*;
    pub fn read_file(file_name: &str, error_message: &str) -> String {
        let mut file = File::open(file_name).expect(error_message);
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    }
}

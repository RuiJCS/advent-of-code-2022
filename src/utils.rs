pub mod utils {
    use std::fs::File;
    use std::io::prelude::*;
    pub fn read_file(file_name: &str, error_message: &str) -> String {
        let mut file = File::open(file_name).expect(error_message);
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    }

    pub fn process_lines<F>(file: &String, mut f: F) -> Vec<u32>
    where
        F: FnMut(&str) -> u32,
    {
        let mut res = vec![0, 0, 0];
        for elf_list in file.lines() {
            let val = f(elf_list);
            res.push(val);
        }
        res
    }
}

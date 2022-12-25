pub mod utils {
    use std::cmp::Ordering;
    use std::fs::File;
    use std::io::prelude::*;

    #[derive(Clone, PartialEq, Eq, Debug, Hash)]
    pub struct Node {
        pub id: String,
        pub flow: u32,
        pub neighbors: Vec<String>,
    }

    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.flow.cmp(&other.flow))
        }
    }

    impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering {
            self.flow.cmp(&other.flow)
        }
    }

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

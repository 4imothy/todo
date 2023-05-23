use crate::searcher::DirPointer;
use crate::searcher::File;

pub fn print_directory(dir_ptr: DirPointer, depth: usize) {
    let dir = dir_ptr.borrow();

    // let new_d_ref = d_ref.borrow().parent.clone().unwrap();
    // d_ref = new_d_ref
    for _ in 0..depth {
        print!("  ");
    }
    println!("Dir: {}", dir.name);

    for child in dir.children.clone() {
        print_directory(child, depth + 1);
    }
    for file in &dir.found_files {
        print_file(file, depth + 1);
    }
}

pub fn print_file(file: &File, depth: usize) {
    for _ in 0..depth {
        print!("  ");
    }
    println!("File: {}", file.name);
    for line in &file.lines {
        for _ in 0..depth {
            print!("  ");
        }
        print!("  ");
        println!("line: {}", line);
    }
}

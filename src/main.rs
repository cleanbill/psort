use std::env;
use std::fs::{read_dir, DirEntry, create_dir_all, copy};
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

fn check_dir(root: &str,entry: DirEntry, src_dir: &str, dst_dir: &str) {
    let sub_dir = Path::new(src_dir).join(entry.file_name());
    let new_src_dir = sub_dir.into_os_string().into_string().unwrap();
    let r = dir_walk(root,&new_src_dir, &dst_dir);
    if r.is_err() {
        let err = r.unwrap_err();
        match err.kind() {
            std::io::ErrorKind::AlreadyExists => println!("already there"),
            _ => todo!(),
        }
    }
}

fn copy_instead() -> bool {
    if let Some(arg1) = env::args().nth(1) {
        return arg1.contains("--copy");
    }
    return false;
}

fn dir_process(root: &str,entry: DirEntry, src_dir: &str, dst_dir: &str) {
    let src_path = &entry.path();
    let name = entry
        .path()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();

    let typ = entry.file_type();
    if typ.is_err() {
        let err = typ.unwrap_err();
        match err.kind() {
            std::io::ErrorKind::AlreadyExists => println!("already there"),
            _ => todo!(),
        }
        return;
    }
    let is_dir = typ.unwrap().is_dir();
    if is_dir {
        check_dir(root,entry, src_dir, dst_dir);
        return;
    }
    if !src_dir.contains("_picked") {
        return;
    }

    // mkdir if needed for root
    let new_path = Path::new(&dst_dir).join(root);
    let dst_path = new_path.as_path();
    let cra = create_dir_all(dst_path);
    if cra.is_err() {
        let err = cra.unwrap_err();
        match err.kind() {
            std::io::ErrorKind::AlreadyExists => println!("Dir already there"),
            _ => print!("{}",err),
        }
    }    
    let dst_path = Path::new(dst_path).join(entry.file_name());
    print!("File found {} ", name);
    file_action(src_path, dst_path, dst_dir,root);
}

fn file_copy(src_path: &PathBuf, dst_path: PathBuf, dst_dir: &str, root: &str){
    let r = copy(&src_path, dst_path);
    if r.is_err() {
        let err = r.unwrap_err();
        match err.kind() {
            std::io::ErrorKind::AlreadyExists => println!("and file already there"),
            _ => todo!(),
        }
        return;
    } 
    println!("and copied file in {}/{} dir",dst_dir,root);

}

fn file_action(src_path: &PathBuf, dst_path: PathBuf, dst_dir: &str, root: &str) {
    if copy_instead(){
        file_copy(src_path, dst_path, dst_dir, root);
        return;
    }
    let r = symlink(&src_path, dst_path);
    if r.is_err() {
        let err = r.unwrap_err();
        match err.kind() {
            std::io::ErrorKind::AlreadyExists => println!("and symbolic link already there"),
            _ => todo!(),
        }
        return;
    } 
    println!("and added symbolic link in {}/{} dir",dst_dir,root);
    
}

fn dir_walk(current_route: &str, src_dir: &str, dst_dir: &str) -> std::io::Result<()> {
    println!("==================================================");
    println!("src is {}", src_dir);
    println!("");

    for entry in read_dir(&src_dir).unwrap() {
        let entry = entry?;
        let root = entry
        .path()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();
         if current_route.len() == 0 {
            dir_process(&root, entry, src_dir, dst_dir);
         } else {
            dir_process(&current_route, entry, src_dir, dst_dir);
         }
    }
    return Ok(());
}

fn main() -> std::io::Result<()> {

    // @TODO use params? https://docs.rs/clap/latest/clap/ and have switch to copy,
    //       rather than symlink
    let src_dir = "/mnt/drive/backup/pictures/years/";
    let dst_dir = "/mnt/drive/backup/pictures/sorted";

    println!("Welcome to picture sort");

    return dir_walk("",src_dir, dst_dir);
}

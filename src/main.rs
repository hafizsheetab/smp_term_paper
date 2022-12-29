
use std::{fs, env};
use smp_term_paper;

fn main() {
    // let args: Vec<String> = vec!["filename".to_owned(), "C:\\Users\\User\\Documents\\GitHub\\Order-Bot\\client\\src".to_owned()];
    // let args: Vec<String> = vec!["filename".to_owned(), "C:\\Users\\User\\Documents\\GitHub\\Have-E-Art\\client\\src".to_owned()];
    // let args: Vec<String> = vec!["filename".to_owned(), "C:\\Users\\User\\Desktop\\Work\\HealthDiary\\healthdiary\\src".to_owned()];
    let args: Vec<String> = env::args().collect();
    if(args.len() != 2){
        panic!("Inappropriate Amount of Args Given. Please Give A Name")
    }
    let program_args: Vec<String> = vec![args[1].to_owned(),env::current_dir().unwrap().as_os_str().to_str().unwrap().to_owned()];
    smp_term_paper::run(&program_args);
    // println!("{}", env::current_dir().unwrap().display());
    // dbg!(args);
    
    // let character = '\0';
    // print!("{character}");
}

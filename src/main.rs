use clap::Parser;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::process::exit;

/// To do program
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Adding todo list
    #[arg(short, long)]
    add: Option<String>,
    ///Checks off the task in given index
    #[arg(short, long)]
    tick: Option<i32>,
    ///Clears the to-do list
    #[clap(short, long)]
    clear: bool,
}

fn main() {
    let cli = Cli::parse();
    let mut todo: Option<String> = cli.add;
    let delind = cli.tick;
    let mut file = fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open("todo.txt")
        .unwrap();
    let mut list = String::new();
    let mut towrite = String::new();
    let mut stringarr: Vec<String> = Vec::new();

    //Reading the file
    file.read_to_string(&mut list)
        .expect("Error Reading the file");

    //convert String to Vector<Strings>
    let mut taskk = String::new();
    let mut flag = false;

    for i in 0..list.len() {
        match list.chars().nth(i) {
            Some('\n') => {
                stringarr.push(taskk.clone());
                taskk.clear();
            }
            None => {
                break;
            }
            _ => taskk.push(list.chars().nth(i).unwrap()),
        }
    }
    //if Add arguement is present then write to the buffer
    if let Some(todo_item) = todo.take() {
        stringarr.push(format!("[ ] {}", todo_item));
        flag = true;
    }

    // if delind arguement is called tick the task with given index
    match delind {
        Some(mut x) if x <= stringarr.len() as i32 => {
            x -= 1;
            if stringarr[x as usize].chars().nth(1).unwrap() != '✅' {
                stringarr[x as usize].replace_range(1..2, "✅");
                flag = true;
            } else {
                println!("Already marked Done");
                exit(0);
            }
        }
        None => (),
        _ => {
            panic!("Arguement out of bound")
        }
    }

    //clear function
    if cli.clear == true {
        let mut file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .open("todo.txt")
            .unwrap();

        file.write(b"").expect("Error while Clearing");

        println!("To-Do List Cleared");
        exit(0);
    }

    //saving changes to .txt file
    if flag {
        let mut file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .open("todo.txt")
            .unwrap();
        for i in 0..stringarr.len() {
            towrite.push_str(&format!("{}\n", stringarr[i]));
        }

        file.write(format!("{}", towrite).as_bytes())
            .expect("Error in Updating Tasks in File");
        println!("{}", towrite);
    } else {
        println!("Pending Tasks are :");
        println!("{}", list);
    }
}

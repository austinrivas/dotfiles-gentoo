use dirs;
use std::fs;
use std::path::Path;

mod dotfiles;
use dotfiles::shell::Command;

fn create_dir() {
    let mut home_path = match dirs::home_dir() {
        Some(path) => path,
        None => panic!("home_dir is not defined in this environment")
    };

    home_path.push("some/dir");

    fs::create_dir_all(&home_path)
        .expect("could not create dir");

    println!("created directory: {:?}", &home_path);

    assert!(Path::new(&home_path).exists());
}

fn sync_package_repos(package_manager: &str) {
    let sync_package_db = Command::new(
        package_manager,
        &vec!["-Sy"] 
    );

    println!("Syncronizing package db.");

    let output = sync_package_db.run();

    output.log();

    assert!(output.success());
}

fn install_git(package_manager: &str) {
    let install_git = Command::new(
        package_manager,
        &vec!["-S", "git", "--noconfirm"]
    );

    println!("Installing git.");

    let output = install_git.run();

    output.log();

    assert!(output.success());
}

// (Full example with detailed comments in examples/01d_quick_example.rs)
//
// This example demonstrates clap's full 'custom derive' style of creating arguments which is the
// simplest method of use, but sacrifices some flexibility.
use clap::Clap;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(version = "0.1", author = "Austin Rivas <austinrivas@gmail.com>")]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long, default_value = "default.conf")]
    config: String,
    /// Some input. Because this isn't an Option<T> it's required to be used
    #[clap(validator(is_png))]
    input: String,
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(version = "1.3", author = "Someone E. <someone_else@other.com>")]
    Test(Test),
    #[clap(version = "0.1", author = "Someone E. <someone_else@other.com>")]
    Install(Install)
}

/// A subcommand for controlling testing
#[derive(Clap)]
struct Test {
    /// Print debug info
    #[clap(short, long)]
    debug: bool
}

/// A subcommand for installing deps
#[derive(Clap)]
struct Install {
    /// Print debug info
    #[clap(short, long)]
    debug: bool
}

fn main() {
    let opts: Opts = Opts::parse();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    println!("Value for config: {}", opts.config);
    println!("Using input file: {}", opts.input);

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match opts.verbose {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    match opts.subcmd {
        SubCommand::Test(t) => {
            if t.debug {
                println!("Printing debug info...");
            } else {
                println!("Printing normally...");
            }
        },
        SubCommand::Install(i) => {
            if i.debug {
                println!("Printing debug info...");
            }
            
            let package_manager = String::from("pacman");
            sync_package_repos(&package_manager);
            install_git(&package_manager);
        }
    }

    // more program logic goes here...
}


fn is_png(val: &str) -> Result<(), String> {
    // val is the argument value passed in by the user
    // val has type of String.
    if val.ends_with(".png") {
        Ok(())
    } else {
        // clap automatically adds "error: " to the beginning
        // of the message.
        Err(String::from("the file format must be png."))
    }
    // Of course, you can do more complicated validation as
    // well, but for the simplicity, this example only checks
    // if the value passed in ends with ".png" or not.
}
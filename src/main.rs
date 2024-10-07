use std::io::{self,Read};
use clap::Parser;
use colored::Colorize;
use anyhow::{Context,Result};
#[derive(Parser)]

struct Options{
#[clap(default_value="ouuups")]
/// what does the horse say ?
message:String,
#[clap(short='d',long="dead")]
///Active dead eye
dead:bool,
#[clap(short='f',long="file")]
///Input file to read
 file:Option<std::path::PathBuf>,
#[clap(short='i',long="stdin")]
 /// Pipening input from an another program
 stdin:bool,
}

fn main() -> Result<(),anyhow::Error> {
let option= Options::parse();
let mut message = String::new();
         if option.stdin{
         io::stdin().read_to_string(&mut message)?;}
           else{message= option.message;}
let eye=if option.dead{"x"} else{"o"};
match &option.file{
     Some(path) =>{
                     let  horse_file=std::fs::read_to_string(path)
                                                   .with_context(|| format!("coun't read file type {:?}",path))?;
                     let eye=format!("{}",eye.red());
                     let message = format!("{}",message.blue());
                     let mut horse_capture=horse_file.replace("{eye}",&eye);
                             horse_capture=horse_capture.replace("{message}",&message);
                    
                     println!("{}",horse_capture);     
                         },
     None        => println!("no thing!")
        }
Ok(())
}




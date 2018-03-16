
// 1323685428145.jpg
extern crate rand;
#[macro_use] extern crate clap;
extern crate chrono;
extern crate clipboard;

use std::io;
use std::fs;
use std::path::Path;
use rand::Rng;
use chrono::prelude::*;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;


fn main() {
  match timestamp() {
   Ok(_) => (),
   Err(err) => {
    // -404: Missing argument
    // -400: Wrongly formatted argument 
    // -406 Incorrect argument- file does not exist and is not directory

    // -502 Unreachable state
    eprintln!("error: {:?}", err);
    std::process::exit(-1);
  }
}
}

fn timestamp() -> Result<(), io::Error>{
  let matches = clap_app!(timestamp =>
    (version: "0.15")
    (author: "azunymous <azu@azunymo.us>")
    (about: "Unix Timestamp Utility - \n 
      Lets you generate, check and rename files with unix timestamps")
        // (@arg CONFIG: -c --config +takes_value "Sets a custom config file") // Set ms or s?
        (@subcommand generate =>
          (about: "Generates a Unix timestamp for today or a given date & time - 'YYYY-MM-DD H:M:S.ms'")
          (@arg date: [DATE] "A date&time: 'YYYY-MM-DD H:M:S.ms' or leave blank for today")
          (@arg clipboard: -c --clipboard "Copy to the clipboard")
          )
        (@subcommand check =>
          (about: "Converts a Unix time stamp or Unix timestamped filename to YYYY-MM-DD H:M:S.ms")
          (@arg filename: <FILENAME> "Unix time stamp or file with a Unix filename")
          (@arg clipboard: -c --clipboard "Copy to the clipboard")
          )
        (@subcommand rename =>
          (about: "Rename file to Unix file name")
          (@arg file: <FILE> "Enter file path to rename to unix filename")
          (@arg date: -d --date +takes_value conflicts_with[stamp] "Enter date" )
          (@arg stamp: -u --unix +takes_value conflicts_with[date] "Enter Unix time stamp")
          (@arg randomize: -r --randomize "Randomizes values around given date/today")
          )
        ).get_matches();

   // Initialize clipboard
   let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

   match matches.subcommand() {
    ("generate", Some(generate_matches)) =>{
        // Check if date is provided
        if let Some(date) = generate_matches.value_of("date") {
          println!("Generating {}", date);
          let unixdur = to_unix(date.to_owned())?;
          if generate_matches.is_present("clipboard") {
            ctx.set_contents(unixdur.to_string()).expect("Can't copy to clipboard!");
          }

        } else {
          let now = Utc::now().format("%Y-%m-%d %H:%M:%S%.f").to_string();
          let unixdur = to_unix(now).expect("Wrong format?");
          println!("Generating today:\n{}", unixdur.to_string());

          if generate_matches.is_present("clipboard") {
            ctx.set_contents(unixdur.to_string()).expect("Can't copy to clipboard!");
          }
        }

      },
      ("check", Some(check_matches)) =>{
            // Now we have a reference to check's matches
            // filename is safe to unwrap as it is required argument
            match Path::new(check_matches.value_of("filename").unwrap()).file_stem() {
              None => println!("No filename in path!"),
              Some(name) => {
                // 1 ms * 1000000 = 1 nanosec
                let path = name.to_string_lossy();
                println!("Input file/timestamp: {:?}", path );
                let unixnano: u32;
                let unixsecs: i64;
                match path.parse::<i64>() {
                  Ok(n) => {
                    unixnano = ((n % 1000)* 1000000) as u32;
                    unixsecs = (n-(n%1000))/1000;
                  },
                  Err(_e) => {
                    println!("Please enter a timestamp");
                    return  Err(io::Error::new(io::ErrorKind::Other,"-400 Wrongly formatted timestamp"))
                  },
                };
                println!("{}s and {} ms",unixsecs,unixnano/1000000 );
                let dt: DateTime<Local> = Local.timestamp(unixsecs, unixnano);
                println!("{}", dt.format("%Y-%m-%d %H:%M:%S%.f"));

                if check_matches.is_present("clipboard") {
                  ctx.set_contents(dt.format("%Y-%m-%d %H:%M:%S%.f").to_string()).expect("Can't copy to clipboard!");
                }
              },
              
            };
            

          },
          ("rename", Some(rename_matches)) =>{
            let ts: u64 = if rename_matches.is_present("date") {
              to_unix(rename_matches.value_of("date").unwrap().to_owned())
              .expect("Wrong format!")
            } else if rename_matches.is_present("stamp") {
              match rename_matches.value_of("stamp").unwrap().parse() {
                Ok(n) => n,
                Err(_) => return Err(io::Error::new(io::ErrorKind::Other,"-406 Could not parse timestamp")),
              }
            } else {
              to_unix((Utc::now().format("%Y-%m-%d %H:%M:%S%.f").to_string())).expect("Wrong format?")
            };


            
            let unix: u64 = if rename_matches.is_present("randomize") {
             gentimestamp(ts, 100)
           } else {
            ts
          };

          let path = Path::new(rename_matches.value_of("file").unwrap());
          if path.is_file() {
            print!("Renaming file: {:?} ", path );
            // extension
            let mut output = path.to_path_buf();
            output.set_file_name(unix.to_string());
            if let Some(ext) = path.extension() {
              output.set_extension(ext);
            } 
            println!("to {}", output.to_string_lossy() );
            fs::rename(path, output).expect("Could not rename!");
          } else if path.is_dir() {
            unimplemented!();
          } else {
              // Not a file
              return Err(io::Error::new(io::ErrorKind::Other,"-406 Not a file"))
            }
            
          },
        ("", None)   => {
          println!("No command was used"); // If no subcommand was usd it'll match the tuple ("", None)
          println!("Type timestamp -h for help");
        },
        _            => return Err(io::Error::new(io::ErrorKind::Other,"-502 Unreachable'")), // If all subcommands are defined above, anything else is unreachabe!()
      }

      Ok(())
    }


    fn gentimestamp(ts: u64, range: i64) -> u64 {
      let variation = rand::thread_rng().gen_range(-range, range);
      if ((ts as i64) + variation) < 0 {
        0
      } else {
        ((ts as i64) + variation) as u64
      }
    }

    fn to_unix(date: String) -> Result<u64, io::Error> {
      let unix_epoch_dt = Utc.ymd(1970, 01, 01).and_hms_milli(00, 00, 00,00);
      match Utc.datetime_from_str(&date, "%Y-%m-%d %H:%M:%S%.f")
      {
        Ok(v) => {
          let unixdur = v.signed_duration_since(unix_epoch_dt);
          Ok(unixdur.num_milliseconds() as u64)
        },
        Err(_e) => Err(io::Error::new(io::ErrorKind::Other,"Not formatted in '%Y-%m-%d %H:%M:%S%.f'")),
      }
    }


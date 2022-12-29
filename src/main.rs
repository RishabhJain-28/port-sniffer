use std::io::{self, Write};
use std::process;
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::{
    env,
    net::{IpAddr, TcpStream},
};

const DEFAULT_THREADS: u16 = 1000;
const MAX_PORT: u16 = 65535;

fn print_help() {
    println!(
        "Enter a ip address as an argument at the end of the flags.\r\nFlags: -j to select how many threads you want\r\n   -h or -help to show help message.\n"
    )
}
struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        // args.next();

        if args.len() < 2 {
            return Err("not enough argumets");
        } else if args.len() > 4 {
            print_help();
            return Err("Too many argumets");
        }

        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments {
                flag: String::from(""),
                ipaddr,
                threads: DEFAULT_THREADS,
            });
        } else {
            let flag = args[1].clone();
            match flag.as_str() {
                "-j" => {
                    let ipaddr = match IpAddr::from_str(&args[3]) {
                        Ok(s) => s,
                        Err(_) => return Err("Not a valid IPADDR; must be a valid IPv4 or IPv6."),
                    };
                    let threads = match args[2].parse::<u16>() {
                        Ok(s) => s,
                        Err(_) => return Err("Fialed to parse thread number"),
                    };
                    Ok(Arguments {
                        flag: String::from("-j"),
                        ipaddr,
                        threads,
                    })
                }
                "-h" | "-help" => {
                    print_help();
                    return Err("help");
                }
                _ => {
                    print_help();
                    return Err("Invalid Syntax");
                }
            }
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }
        if (MAX_PORT - port) <= num_threads {
            break;
        }
        port += num_threads;
    }
}
fn main() {
    let input_args: Vec<String> = env::args().collect();
    let arguments = Arguments::new(&input_args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        }
        eprintln!("Problem parsing argumants: {err}");
        process::exit(1)
    });
    let num_threads = arguments.threads;
    let addr = arguments.ipaddr;
    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx, i, addr, num_threads);
        });
    }
    drop(tx);
    let mut out = vec![];
    for r in rx {
        out.push(r)
    }
    println!("");
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
}

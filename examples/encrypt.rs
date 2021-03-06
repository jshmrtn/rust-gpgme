extern crate getopts;
extern crate gpgme;

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process::exit;

use getopts::{HasArg, Occur, Options};

use gpgme::{Context, Protocol};

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options] FILENAME", program);
    eprintln!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let program = &args[0];

    let mut opts = Options::new();
    opts.optflag("h", "help", "display this help message");
    opts.optflag("", "openpgp", "use the OpenPGP protocol (default)");
    opts.optflag("", "cms", "use the CMS protocol");
    opts.opt(
        "r",
        "recipient",
        "encrypt message for NAME",
        "NAME",
        HasArg::Yes,
        Occur::Multi,
    );

    let matches = match opts.parse(&args[1..]) {
        Ok(matches) => matches,
        Err(fail) => {
            print_usage(program, &opts);
            eprintln!("{}", fail);
            exit(1);
        }
    };

    if matches.opt_present("h") {
        print_usage(program, &opts);
        return;
    }

    if matches.free.len() != 1 {
        print_usage(program, &opts);
        exit(1);
    }

    let proto = if matches.opt_present("cms") {
        Protocol::Cms
    } else {
        Protocol::OpenPgp
    };

    let mut ctx = Context::from_protocol(proto).unwrap();
    ctx.set_armor(true);

    let recipients = matches.opt_strs("r");
    let keys = if !recipients.is_empty() {
        ctx.find_keys(recipients)
            .unwrap()
            .filter_map(Result::ok)
            .filter(|k| k.can_encrypt())
            .collect()
    } else {
        Vec::new()
    };

    let mut input = File::open(&matches.free[0]).unwrap();
    let mut output = Vec::new();
    ctx.encrypt(&keys, &mut input, &mut output)
        .expect("encrypting failed");

    println!("Begin Output:");
    io::stdout().write_all(&output).unwrap();
    println!("End Output.");
}

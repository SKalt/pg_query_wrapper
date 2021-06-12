

use std::process::exit;

use clap;

fn main() {
    let matches = clap::App::new("prost_build")
        .arg(
            clap::Arg::with_name("include")
                .long("include").short("I")
                .takes_value(true)
                .multiple(true)
            )
        .arg(clap::Arg::with_name("targets").multiple(true).required(true))
        .arg(clap::Arg::with_name("outdir").long("--out-dir").short("-o").default_value("."))
        .get_matches();
    let protos  = matches.values_of("targets")
        .unwrap()
        .collect::<Vec<&str>>();
    let includes = matches.values_of("include")
        .unwrap_or_default()
        .collect::<Vec<&str>>();
    let outdir = matches.value_of("outdir").unwrap();
    let mut cfg = prost_build::Config::new();
    cfg.out_dir(outdir);
    match cfg.compile_protos(protos.as_slice(), includes.as_slice()) {
        Ok(_) => {},
        Err(err) => {
            println!("{}", err);
            exit(1);
        },
    }
}

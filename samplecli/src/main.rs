use clap::{App,Arg};

fn main(){
    let matches = App::new("My RPN program")
    // .version("1.0.0")
    .long_version(
        "v1.1.0
         commit: abcdef89726d
         revision: 123
         release: 2
         binary: myprog")
    .author("Your name")
    .about("Super awesome sample RPN calculator")
    .arg(
        Arg::new("formula_file")
        .help("Formulas written in RPN")
        .value_name("FILE1")
        .index(1)
        .required(false),
    )
    .arg(
        Arg::new("verbose")
        .help("Sets the lovel of verbosity")
        .short('v')
        .long("verbose")
    )
    .get_matches();

    match matches.value_of("formula_file"){
        Some(file) => println!("Filespecified: {}",file),
        None => println!("No file specified."),
    };
    let verbose = matches.is_present("verbose");
    println!("Is verbosity specified? :{}",verbose);

    // println!("aaabcdkldad");
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}",args);
}
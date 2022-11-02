use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Input text 
    text: Vec<String>,
    
    /// Do not print newline
    #[arg(short='n')]
    flag: bool
}


fn main() {
    let args = Args::parse();
     // window 会按 /r/n 分行   linux 按 /n 分行
    let ending = if args.flag { "" } else { "\r\n" };
    print!("{}{}", args.text.join(" "), ending);

}

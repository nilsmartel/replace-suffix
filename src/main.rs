fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() == 0 || args[0] == "--help" || args.len() > 2 {
        print_help();
    }

    let newsuffix = args.pop().unwrap();
    let inputfile = args.pop().unwrap();

    let result = replace_suffix(inputfile, newsuffix);
    if result.is_err() {
        println!("failed to rename file");
        std::process::exit(1);
    }
}

fn print_help() -> ! {
    println!("replace-suffix <inputfile> <newsuffix>");
    std::process::exit(1);
}

fn replace_suffix(file: String, suf: String) -> std::io::Result<()> {
    let new_name = {
        let pos = file.rfind(".");
        if pos.is_none() {
            // nothing to do here
            std::process::exit(0);
        }
        let pos = pos.unwrap();

        format!("{}.{}", &file[..pos], suf)
    };

    std::fs::rename(file, new_name)
}

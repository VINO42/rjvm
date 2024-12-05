use std::env;
pub struct cmd<String> {
    help_flag: bool,
    version_flag: bool,
    cp_option: String,
    class: String,
    args: Vec<String>,
}

impl<String> cmd<String> {
    fn parse_cmd() {
        let mut cmd_obj=cmd{
            help_flag: false,
            version_flag: false,
            cp_option: (),
            class: (),
            args: vec![],
        };
        // let args:Vec<String> =env::args().collect();

    }
}
fn print_usage(){
    let args:Vec<String> = env::args().collect();

    println!("Usage: {} [-options] class [args...] \n",args[0])
}
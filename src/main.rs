use clap::App;

fn main() {
    let app = App::new("Auto domains blocker")
        .version("0.1")
        .author("Avimitin <avimitin@gmail.com>")
        .about("This app help you get rid of internet addiction")
        .subcommand(App::new("block")
            .about("block all the domains now"))
        .get_matches();

    if let Some(_) = app.subcommand_matches("block") {
        println!("Running block process");
    }
}


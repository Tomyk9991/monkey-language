pub fn help_screen() -> String {
    let monkey_logo = r#"
  .--.  .-'''-.  .--.
 /."".v'.-. .-.`v.""\\
 ||  / / O| | O\ \  ||       __  __             _                    _
 \\_/| \__| |__/ |\_//      |  \/  | ___  _ __ | | _____ _   _      | |    __ _ _ __   __ _ _   _  __ _  __ _  ___
  `-'\  .-n-n-.  /`-'       | |\/| |/ _ \| '_ \| |/ / _ \ | | |_____| |   / _` | '_ \ / _` | | | |/ _` |/ _` |/ _ \
      \/       \/           | |  | | (_) | | | |   <  __/ |_| |_____| |__| (_| | | | | (_| | |_| | (_| | (_| |  __/
      (\`.___.'/)           |_|  |_|\___/|_| |_|_|\_\___|\__, |     |_____\__,_|_| |_|\__, |\__,_|\__,_|\__, |\___|
       \`.___.'/                                         |___/                        |___/             |___/
        `.___.
"#;
    
    let mut help = String::from("\nMonkey-Language's compiler / interpreter\n\n");
    help.push_str("Usage: monkey-language [OPTIONS] [COMMAND]\n\n");
    help.push_str("Options:\n");

    help.push_str(&format!("{:3}{:13}{:13}{}", " ", "-h, --help",  "<>", "Print help information\n"));
    help.push_str(&format!("{:3}{:13}{:13}{}", " ", "-i, --input", "<FILE>", "Path to the main entry file\n"));


    return format!("{}\n{}", monkey_logo, help);
}
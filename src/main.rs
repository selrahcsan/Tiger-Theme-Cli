use std::env;
mod help;
mod themes;
mod colors;
mod version;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "-t" {
        let system_theme = &args[2];
        let type_themes = &String::from("/Net/IconThemeName");
        themes::themes(system_theme, type_themes);

        let type_icons = &String::from("/Net/ThemeName");
        let icon_theme = &args[2].to_lowercase();
        themes::themes(icon_theme, type_icons);

        let _mode_theme_dark = &String::from("Flat-Remix-Dark-XFWM");
        let _mode_theme_light = &String::from("Flat-Remix-Light-XFWM");
        if icon_theme.find("dark").is_some() {
            themes::mode_themes(_mode_theme_dark);
        } else {
            themes::mode_themes(_mode_theme_light);
        }
    } else if args.len() > 1 && args[1] == "-h" {
        help::help();
    } else if args.len() >1 && args[1] == "-d"{
        colors::disponibles_colors();
    } else if args.len() >1 && args[1] == "-v"{
        version::version();
    } else {
        println!("Use -h para exibir ajuda")
    }
}
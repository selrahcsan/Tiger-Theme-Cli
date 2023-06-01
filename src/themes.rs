use std::process::Command;

pub(crate) fn themes(_theme_name: &String, _theme_type: &String) {
    let status = Command::new("xfconf-query")
        .arg("-c")
        .arg("xsettings")
        .arg("-p")
        .arg(_theme_type)
        .arg("-s")
        .arg(_theme_name)
        .status()
        .expect("failed to execute process");
    assert!(status.success());
}

pub(crate) fn mode_themes(_theme_mode: &String) {
    let status = Command::new("xfconf-query")
        .arg("-c")
        .arg("xfwm4")
        .arg("-p")
        .arg("/general/theme")
        .arg("-s")
        .arg(_theme_mode)
        .status()
        .expect("failed to execute process");
    assert!(status.success());
}
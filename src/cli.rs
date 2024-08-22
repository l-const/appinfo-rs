pub(crate) fn setup() -> &'static str {
    let str_token = r#" 
        fn setup(){
            use clap::Command;
            let author = env!("CARGO_PKG_AUTHORS");
            let pkg_name = env!("CARGO_PKG_NAME");
            let version = env!("CARGO_PKG_VERSION");
            let description = env!("CARGO_PKG_DESCRIPTION");
            let repository = env!("CARGO_PKG_REPOSITORY");
            let detailed_version = format!("version: {version}, commit revision: {version}, repository: {repository}");
            let version: &'static str = String::leak(detailed_version);
            let _matches = Command::new(pkg_name)
                .author(author)
                .version(version)
                .about(description)
                .help_template("{name} ({version}) - {about}")
                .get_matches();
            if std::env::args().len() > 1 {
                return;
            }
        }
    "#;
    return str_token;
}

use clap::Command;

#[derive(Debug)]
struct CargoEnvVars {
    author: &'static str,
    app_name: &'static str,
    version: &'static str,
    commit: &'static str,
    description: &'static str,
    repository: &'static str
}

fn detailed_version(version: &str, repository: &str, commit: &str) -> &str {
    "version: {version}, commit revision: {commit}, repository: {repository}"
}

pub(crate) fn setup() {
    let env_vars = init_env_vars();
    let CargoEnvVars {commit, repository, version , ..} = env_vars;
    let _matches = Command::new(env_vars.app_name)
        .author(env_vars.author)
        .version(detailed_version(version, repository, commit))
        .about(env_vars.description)
        .help_template("{name} ({version}) - {about}")
        .get_matches();

    if std::env::args().len() > 1 {
        return;
    } 
}

fn init_env_vars() -> CargoEnvVars {
    let author = env!("CARGO_PKG_AUTHORS");
    let pkg_name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let description = env!("CARGO_PKG_DESCRIPTION");
    let repository = env!("CARGO_PKG_REPOSITORY");
    CargoEnvVars {
        author,
        app_name,
        version,
        commit: version,
        description,
        repository
    }
}
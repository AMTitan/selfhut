use crate::config::CONFIG;
use crate::git::commits::get_commits;

use crate::git::main_branch::main_branch;
use crate::utils::repo_config::repo_config;
use crate::PathBufWithDotfiles;
use rocket_dyn_templates::{context, Template};

#[get("/<repo>/log?<from>", rank = 2)]
pub fn log_main(repo: String, from: Option<String>) -> Option<Template> {
    let main_branch = main_branch(repo.clone())?;
    let commits = get_commits(repo.clone(), 21, from, None);
    let last_commit = match commits {
        Some(ref commits) => {
            if commits.len() == 21 {
                Some(commits[19].commit_hash.clone())
            } else {
                None
            }
        }
        None => None,
    };
    let commits = match commits {
        Some(mut commits) => {
            if commits.len() == 21 {
                commits.pop();
            }
            Some(commits)
        }
        None => None,
    };
    Some(Template::render(
        "repository/log",
        context! {
            title: format!("/ :: {}", repo),
            repo: repo.clone(),
            config: repo_config(repo),
            domain: CONFIG.domain.to_string(),
            user: CONFIG.name.to_string(),
            active: "log",
            commits,
            branch: main_branch,
            current_dir_file: "/",
            current_dir: "/",
            payment: CONFIG.payment_link.clone(),
            last_commit,
            mailing_list: CONFIG.mailing_list.clone()
        },
    ))
}

#[get("/<repo>/log/<branch>/<location..>?<from>", rank = 2)]
pub fn log(
    repo: String,
    branch: String,
    location: PathBufWithDotfiles,
    from: Option<String>,
) -> Option<Template> {
    let commits = get_commits(
        repo.clone(),
        21,
        Some(from.unwrap_or_else(|| branch.clone())),
        Some(format!("{}", location.get().display()).replace("//", "/")),
    );
    let last_commit = match commits {
        Some(ref commits) => {
            if commits.len() == 21 {
                Some(commits[19].commit_hash.clone())
            } else {
                None
            }
        }
        None => None,
    };
    let commits = match commits {
        Some(mut commits) => {
            if commits.len() == 21 {
                commits.pop();
            }
            Some(commits)
        }
        None => None,
    };
    Some(Template::render(
        "repository/log",
        context! {
            title: format!("/{} :: {}", location.get().display(), repo),
            repo: repo.clone(),
            config: repo_config(repo),
            domain: CONFIG.domain.to_string(),
            user: CONFIG.name.to_string(),
            active: "log",
            commits,
            branch,
            current_dir_file: format!("/{}/", location.get().display()).replace("//", "/"),
            current_dir: format!("/{}", location.get().display()),
            payment: CONFIG.payment_link.clone(),
            last_commit,
            mailing_list: CONFIG.mailing_list.clone()
        },
    ))
}

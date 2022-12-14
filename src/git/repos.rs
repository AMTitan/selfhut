use crate::config::CONFIG;
use crate::utils::repo_config::repo_config;
use serde_derive::Serialize;
use std::fs;

pub fn get_repos() -> Option<Vec<Repo>> {
    let home = CONFIG.git_location.clone();
    Some(sort_modified(
        fs::read_dir(home)
            .ok()?
            .filter_map(|path| path.ok())
            .filter(|path| path.file_type().is_ok())
            .filter(|path| path.file_type().unwrap().is_dir())
            .filter(|path| path.metadata().is_ok())
            .filter(|path| path.metadata().unwrap().modified().is_ok())
            .map(|path| path.file_name().into_string())
            .filter_map(|path| path.ok())
            .filter(|path| !path.starts_with('.'))
            .filter(|path| path.ends_with(".git"))
            .map(|path| {
                let name = path[0..path.len() - 4].to_string();
                Repo {
                    name: name.clone(),
                    description: repo_config(name).description.unwrap_or_default(),
                }
            })
            .collect::<Vec<Repo>>(),
    ))
}

#[derive(Serialize, Clone)]
pub struct Repo {
    pub name: String,
    pub description: String,
}

fn sort_modified(mut repos: Vec<Repo>) -> Vec<Repo> {
    let home = CONFIG.git_location.clone();
    repos.sort_by(|a, b| {
        let mut a_loc = home.clone();
        a_loc.push(format!("{}.git", a.name));
        let mut b_loc = home.clone();
        b_loc.push(format!("{}.git", b.name));
        b_loc
            .metadata()
            .unwrap()
            .modified()
            .unwrap()
            .partial_cmp(&a_loc.metadata().unwrap().modified().unwrap())
            .unwrap()
    });
    repos
}

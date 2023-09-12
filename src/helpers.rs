use std::{
    path::PathBuf,
    str::FromStr,
    fs::{File, create_dir_all},
    io::Write,
};

use slugify::slugify;
use anyhow::{Context, Result, bail};
use git2::{Repository, IndexAddOption};

pub fn type_to_str<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

pub fn sys_base_path() -> PathBuf {
    PathBuf::from_str("ripi").unwrap()
}

pub fn get_closed_dir() -> PathBuf {
    let mut closed = sys_base_path();
    closed.push(".closed");
    closed
}


pub fn write_file(dir: &PathBuf, file: &str, content: Option<&str>) -> Result<()> {
    create_dir_all(dir)
        .with_context(|| format!("Could not create {}",
                                 dir.display()))?;
    let mut file_path = dir.clone();
    file_path.push(file);
    let mut file = File::create(&file_path)
        .with_context(|| format!("Could not create file {}",
                                 &file_path.display()))?;
    if let Some(c) = content {
        file.write_all(c.as_bytes())
            .with_context(|| format!(
                    "Could not write content to file {}",
                    file_path.display()))?;
    }
    Ok(())
}

pub fn is_not_empty(arg: &str) -> Result<String> {
    if arg.is_empty() {
        bail!("issue create: name cannot be empty");
    }
    Ok(arg.to_string())
}

pub fn slug(s: &str) -> String {
    slugify!(&s.to_lowercase(), separator = "_")
}

pub fn get_file_name(path: &PathBuf) -> String {
    path.file_name().unwrap().to_str().unwrap().to_owned()
}

pub fn get_parent_dir(path: &PathBuf) -> String {
    path.parent().unwrap().to_str().unwrap().to_owned()
}

pub fn git_commit(files_to_add: Option<&[String]>, msg: &str) -> Result<()> {
    let repo = Repository::open(".")
        .with_context(|| "failed to open repository")?;
    let signature = repo.signature()?;
    let mut index = repo.index()?;
    if let Some(files_to_add) = files_to_add {
        index.add_all(files_to_add.iter(), IndexAddOption::DEFAULT, None)?;
    }
    index.write()?;
    let oid = index.write_tree()?;
    let tree = repo.find_tree(oid)?;
    let head = repo.head()?;
    let ref_name = head.name();
    let parent_commit_res = head.peel_to_commit();
    let parent_commit = if parent_commit_res.is_ok() {
        vec![parent_commit_res.as_ref().unwrap()]
    } else {
        vec![]
    };

    repo.commit(
        ref_name,
        &signature,
        &signature,
        msg,
        &tree,
        &parent_commit,
    )?;
    Ok(())
}

use core::str;

use bytes::Buf;

use askama_axum::IntoResponse;
use axum::{body::Bytes, routing::post, Router};
use git2::{Repository, TreeWalkMode, TreeWalkResult};
use tar::Archive;

async fn archive_files(file: Bytes) -> impl IntoResponse {
    let mut archive = Archive::new(file.reader());
    archive.entries().unwrap().count().to_string()
}

async fn archive_files_size(file: Bytes) -> impl IntoResponse {
    let mut archive = Archive::new(file.reader());
    archive
        .entries()
        .unwrap()
        .map(|f| f.unwrap().size())
        .sum::<u64>()
        .to_string()
}

async fn cookie(file: Bytes) -> impl IntoResponse {
    let repo_dir = tempfile::tempdir().unwrap();
    let mut archive = Archive::new(file.reader());
    if archive.unpack(repo_dir.as_ref()).is_ok() {
        let repo = Repository::open(repo_dir.path()).unwrap();
        let branch = repo
            .find_branch("christmas", git2::BranchType::Local)
            .unwrap();
        let mut commit = branch.get().peel_to_commit().unwrap();
        while commit.parent_count() > 0 {
            let mut found = false;
            commit
                .tree()
                .unwrap()
                .walk(TreeWalkMode::PreOrder, |_, entry| {
                    if let Some(name) = entry.name() {
                        if name == "santa.txt" {
                            let obj = entry.to_object(&repo).unwrap();
                            let blob = obj.as_blob().unwrap();
                            let content = blob.content();
                            let text = str::from_utf8(content).unwrap();
                            if text.contains("COOKIE") {
                                found = true;
                                TreeWalkResult::Abort
                            } else {
                                TreeWalkResult::Ok
                            }
                        } else {
                            TreeWalkResult::Ok
                        }
                    } else {
                        TreeWalkResult::Ok
                    }
                })
                .unwrap();
            if found {
                break;
            }
            commit = commit.parent(0).unwrap();
        }
        format!("{} {}", commit.author().name().unwrap(), commit.id())
    } else {
        axum::http::StatusCode::INTERNAL_SERVER_ERROR.to_string()
    }
}

pub fn get_routes() -> Router {
    Router::new()
        .route("/20/archive_files", post(archive_files))
        .route("/20/archive_files_size", post(archive_files_size))
        .route("/20/cookie", post(cookie))
}

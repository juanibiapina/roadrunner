extern crate git2;

use std::collections::BTreeSet;

use self::git2::Repository;
use self::git2::Status;
use self::git2::ErrorCode;
use self::git2::BranchType;
use self::git2::Oid;

use types::Integration;
use types::Placeholder;

pub struct Git {
    repo: Repository,
    index: u8,
    wt: u8,
    untracked: u8,
}

impl Git {
    pub fn new() -> Option<Git> {
        Repository::discover(".").ok().map(|repo| {
            let mut index = 0;
            let mut wt = 0;
            let mut untracked = 0;

            {
                let statuses = repo.statuses(None).unwrap();

                let mut status_changed = Status::empty();
                status_changed.insert(Status::WT_MODIFIED);
                status_changed.insert(Status::WT_DELETED);
                status_changed.insert(Status::WT_TYPECHANGE);
                status_changed.insert(Status::WT_RENAMED);

                let mut status_staged = Status::empty();
                status_staged.insert(Status::INDEX_NEW);
                status_staged.insert(Status::INDEX_MODIFIED);
                status_staged.insert(Status::INDEX_DELETED);
                status_staged.insert(Status::INDEX_TYPECHANGE);
                status_staged.insert(Status::INDEX_RENAMED);

                let mut status_untracked = Status::empty();
                status_untracked.insert(Status::WT_NEW);

                for entry in statuses.iter() {
                    if entry.status().intersects(status_changed) {
                        wt += 1;
                    }

                    if entry.status().intersects(status_staged) {
                        index += 1;
                    }

                    if entry.status().intersects(status_untracked) {
                        untracked += 1;
                    }
                }
            }

            Git {
                repo: repo,
                index: index,
                wt: wt,
                untracked: untracked,
            }
        })
    }
}

impl Integration for Git {
    fn eval(&self, placeholder: &Placeholder) -> String {
        match placeholder.0 {
            "head" => {
                if self.repo.head_detached().unwrap() {
                    match self.repo.head() {
                        Ok(head) => {
                            let commit = self.repo.find_commit(head.target().unwrap()).unwrap();
                            commit.as_object().short_id().unwrap().as_str().unwrap().to_owned()
                        },
                        Err(_) => panic!("invalid git head"),
                    }
                } else {
                    match self.repo.head() {
                        Ok(head) => head.shorthand().unwrap().to_owned(),
                        Err(ref e) if e.code() == ErrorCode::UnbornBranch => "UNBORN".to_owned(),
                        Err(_) => panic!("invalid git head"),
                    }
                }
            },
            "commits" => {
                if self.repo.head_detached().unwrap() {
                    return "".to_owned();
                }

                match self.repo.head() {
                    Ok(head) => {
                        if ! head.is_branch() {
                            return "".to_owned();
                        }

                        let branch_name = head.name().unwrap();
                        let branch = self.repo.find_branch(&branch_name[11..], BranchType::Local).unwrap();
                        let remote_branch = branch.upstream().unwrap();

                        let left_id = branch.get().target().unwrap();
                        let right_id = remote_branch.get().target().unwrap();

                        let merge_base_id = self.repo.merge_base(left_id, right_id).unwrap();

                        let mut revwalk_left = self.repo.revwalk().unwrap();
                        revwalk_left.push(left_id).unwrap();
                        revwalk_left.push(merge_base_id).unwrap();

                        let mut revwalk_right = self.repo.revwalk().unwrap();
                        revwalk_right.push(right_id).unwrap();
                        revwalk_right.push(merge_base_id).unwrap();

                        let local_commits = revwalk_left.map(|r| r.unwrap()).collect::<BTreeSet<Oid>>();
                        let remote_commits = revwalk_right.map(|r| r.unwrap()).collect::<BTreeSet<Oid>>();

                        let ahead = local_commits.difference(&remote_commits).count();
                        let behind = remote_commits.difference(&local_commits).count();

                        let mut result = String::new();

                        if behind > 0 {
                            result.push_str(&format!("↓{}", behind));
                        }

                        if ahead > 0 {
                            result.push_str(&format!("↑{}", ahead));
                        }

                        result
                    },
                    Err(ref e) if e.code() == ErrorCode::UnbornBranch => "".to_owned(),
                    Err(_) => panic!("invalid git head"),
                }
            },
            "index" => {
                if self.index > 0 {
                    format!("●{}", self.index)
                } else {
                    "".to_owned()
                }
            },
            "wt" => {
                if self.wt > 0 {
                    format!("+{}", self.wt)
                } else {
                    "".to_owned()
                }
            },
            "untracked" => {
                if self.untracked > 0 {
                    "…".to_owned()
                } else {
                    "".to_owned()
                }
            },
            "clean" => {
                if self.index == 0 && self.wt == 0 && self.untracked == 0 {
                    "✓".to_owned()
                } else {
                    "".to_owned()
                }
            },
            _ => panic!("unsupported integration placeholder"),
        }
    }
}

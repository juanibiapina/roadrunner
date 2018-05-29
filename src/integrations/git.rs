extern crate git2;

use self::git2::Repository;
use self::git2::Status;
use self::git2::ErrorCode;

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
                match self.repo.head() {
                    Ok(head) => head.shorthand().unwrap().to_owned(),
                    Err(ref e) if e.code() == ErrorCode::UnbornBranch => "UNBORN".to_owned(),
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

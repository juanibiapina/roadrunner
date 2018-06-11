extern crate rlua;

use self::rlua::{UserData, UserDataMethods};
use nom;
use nom::types::CompleteStr;

use std::process::Command;

fn as_str(result: CompleteStr) -> &str {
    result.0
}

named!(header_head<CompleteStr, &str>, map!(delimited!(tag!("# branch.head "), is_not!("\n"), eof!()), as_str));
named!(header_ab<CompleteStr, (u8, u8)>,
    delimited!(
        tag!("# branch.ab "),
        map!(
            separated_pair!(
                preceded!(char!('+'), nom::digit),
                char!(' '),
                preceded!(char!('-'), nom::digit)
            ),
            |(ahead, behind)| (ahead.0.parse().unwrap(), behind.0.parse().unwrap())
        ),
        eof!()
    )
);

#[derive(Default, Clone)]
pub struct Git {
    enabled: bool,
    head: String,
    ahead: i64,
    behind: i64,
    index: i64,
    wt: i64,
    untracked: i64,
}

impl UserData for Git {
    fn add_methods(methods: &mut UserDataMethods<Self>) {
        methods.add_method("enabled", |_, git, ()| { Ok(git.enabled()) });
        methods.add_method("head", |_, git, ()| { Ok(git.head()) });
        methods.add_method("ahead", |_, git, ()| { Ok(git.ahead()) });
        methods.add_method("behind", |_, git, ()| { Ok(git.behind()) });
        methods.add_method("index", |_, git, ()| { Ok(git.index()) });
        methods.add_method("wt", |_, git, ()| { Ok(git.wt()) });
        methods.add_method("untracked", |_, git, ()| { Ok(git.untracked()) });

    }
}

impl Git {
    pub fn new() -> Git {
        let output = Command::new("git")
            .arg("status")
            .arg("--porcelain=2")
            .arg("--branch")
            .output()
            .expect("failed to execute git process");

        if !output.status.success() {
            return Git::default();
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        let lines = stdout.split("\n");

        let mut head = "";
        let mut ahead = 0;
        let mut behind = 0;
        let mut index = 0;
        let mut wt = 0;
        let mut untracked = 0;

        for line in lines {
            if line.starts_with("#") {
                if let Ok((CompleteStr(""), value))  =  header_head(CompleteStr(line)) {
                    head = value;
                    continue;
                }

                if let Ok((CompleteStr(""), (v1, v2)))  =  header_ab(CompleteStr(line)) {
                    ahead = v1 as i64;
                    behind = v2 as i64;
                    continue;
                }

                continue;
            }

            if line.starts_with("1") || line.starts_with("2") {
                let line = &line[2..4];

                let mut chars = line.chars();
                let first = chars.next().unwrap();
                let second = chars.next().unwrap();

                if first != '.' {
                    index += 1;
                }

                if second != '.' {
                    wt += 1;
                }

                continue;
            }

            if line.starts_with("?") {
                untracked += 1;

                continue;
            }
        }

        Git {
            enabled: true,
            head: head.to_owned(),
            ahead,
            behind,
            index,
            wt,
            untracked,
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn head(&self) -> String {
        self.head.clone()
    }

    pub fn ahead(&self) -> i64 {
        self.ahead
    }

    pub fn behind(&self) -> i64 {
        self.behind
    }

    pub fn index(&self) -> i64 {
        self.index
    }

    pub fn wt(&self) -> i64 {
        self.wt
    }

    pub fn untracked(&self) -> i64 {
        self.untracked
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_head() {
        assert_eq!(header_head(CompleteStr("# branch.head master")).unwrap(), (CompleteStr(""), "master"));
    }

    #[test]
    fn test_header_ab() {
        assert_eq!(header_ab(CompleteStr("# branch.ab +32 -2")).unwrap(), (CompleteStr(""), (32, 2)));
    }
}

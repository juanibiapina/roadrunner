use nom;
use nom::types::CompleteStr;

use std::fmt::Write;
use std::process::Command;

use types::Integration;
use types::Placeholder;

pub struct Git {
    head: String,
    ahead: u8,
    behind: u8,
    index: u8,
    wt: u8,
    untracked: u8,
}

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

impl Git {
    pub fn new() -> Option<Git> {
        let output = Command::new("git")
            .arg("status")
            .arg("--porcelain=2")
            .arg("--branch")
            .output()
            .expect("failed to execute git process");

        if !output.status.success() {
            return None;
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
                    ahead = v1;
                    behind = v2;
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

        Some(Git{
            head: head.to_owned(),
            ahead: ahead,
            behind: behind,
            index: index,
            wt: wt,
            untracked: untracked,
        })
    }
}

impl Integration for Git {
    fn eval(&self, placeholder: &Placeholder) -> String {
        match placeholder.0 {
            "head" => {
                self.head.to_string()
            },
            "commits" => {
                let mut result = String::new();

                if self.behind > 0 {
                    write!(result, "↓{}", self.behind).unwrap();
                }

                if self.ahead > 0 {
                    write!(result, "↑{}", self.ahead).unwrap();
                }

                result
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

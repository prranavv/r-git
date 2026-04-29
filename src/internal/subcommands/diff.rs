use std::{io, path::PathBuf};
use crossterm::style::Stylize;
use std::io::Write;
use crate::{Result, error::RGitError, internal::utils::{parse_index, parse_working_dir, read_object}};

pub fn diff(file_path:&PathBuf)->Result<()>{
    let working_dir = parse_working_dir()?;
    let index = parse_index()?;

    let w = working_dir.iter().find(|e|e.file_path==*file_path).unwrap();
    let i = index.iter().find(|e|e.file_path==*file_path).ok_or(RGitError::UnTrackedFile)?;
    let w = &w.hash;
    let i =&i.hash;
    let (_obj_type,w_content) = read_object(w)?;
    let (_obj_type,i_content) = read_object(i)?;

    let new: Vec<&str> = w_content.lines().collect();
    let old: Vec<&str> =i_content.lines().collect();

    let m = old.len();
    let n = new.len();

    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 0..m {
        for j in 0..n {
            if old[i] == new[j] {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = dp[i + 1][j].max(dp[i][j + 1]);
            }
        }
    }

    let mut i = m;
    let mut j = n;
    let mut result = Vec::new();

    while i > 0 && j > 0 {
        if old[i - 1] == new[j - 1] {
            result.push((" ", old[i - 1]));
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] >= dp[i][j - 1] {
            result.push(("-", old[i - 1]));
            i -= 1;
        } else {
            result.push(("+", new[j - 1]));
            j -= 1;
        }
    }

    while i > 0 {
        result.push(("-", old[i - 1]));
        i -= 1;
    }

    while j > 0 {
        result.push(("+", new[j - 1]));
        j -= 1;
    }

    result.reverse();

    let mut stdout = io::stdout();
    for (tag, line) in result {
        match tag {
            " " => writeln!(stdout," {}", line)?,
            "-" => writeln!(stdout,"-{}", line.red())?,
            "+" => writeln!(stdout,"+{}", line.green())?,
            _ => {}
        }
    }
    Ok(())
}
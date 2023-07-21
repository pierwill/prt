use clap::Parser;
use git2::Repository;

use std::collections::HashMap;
use std::io::{self, Write};
use std::process::Command;

#[derive(Parser)]
struct Cli {
    build: String,
    #[arg(short, long, default_value_t = false)]
    dry_run: bool,
}

fn main() {
    let cli = Cli::parse();

    let build = cli.build;

    let pr_msg = create_pr_msg(build);
    println!("{pr_msg}");

    if !cli.dry_run {
        let output = Command::new("gh")
            .arg("pr")
            .arg("create")
            .arg("--fill")
            .arg("--body")
            .arg(pr_msg.clone())
            .output()
            .expect("failed to execute process");
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        let stderr = String::from_utf8(output.stderr).expect("whoopps");
        if stderr.contains("already exists") {
            println!("sup");
            let output = Command::new("gh")
                .arg("pr")
                .arg("edit")
                .arg("--body")
                .arg(pr_msg)
                .output()
                .expect("failed to execute process");
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
        }
    }
}

fn create_pr_msg(build: String) -> String {
    let mut repo2staging: HashMap<&str, &str> = HashMap::default();
    repo2staging.insert(
        "cloud-docs",
        "https://docs-atlas-staging.mongodb.com/cloud-docs/docsworker-xlarge/",
    );
    repo2staging.insert(
        "mms-docs",
        "https://docs-opsmanager-staging.mongodb.com/docsworker-xlarge/",
    );

    // Get repo workdir
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    let reponame = repo
        .workdir()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    // Get branch name
    let head = match repo.head() {
        Ok(head) => Some(head),
        Err(ref e)
            if e.code() == git2::ErrorCode::UnbornBranch
                || e.code() == git2::ErrorCode::NotFound =>
        {
            None
        }
        Err(e) => panic!("{e}"),
    };
    let branch = head.as_ref().and_then(|h| h.shorthand()).unwrap();

    // Get base URL for staging
    let staging_base = *repo2staging.get(reponame).unwrap_or(&"");
    let mut staging_pr_base = String::from(staging_base);
    staging_pr_base.push_str(branch);

    // Get list of changed files
    let diff = Command::new("git")
        .arg("diff")
        .arg("--name-only")
        .arg("upstream/master")
        .output()
        .expect("failed to execute process");
    let diff = String::from_utf8(diff.stdout).expect("whoops");
    let diff_files: Vec<&str> = diff.trim_end().split("\n").collect();
    let diff_files: Vec<_> = diff_files
        .iter()
        .map(|s| s.replace("source/", ""))
        .map(|s| s.replace(".txt", ""))
        .map(|s| s.replace(".rst", ""))
        .map(|s| s.replace(".yaml", ""))
        .filter(|s| !s.contains("includes"))
        .collect();

    // Build PR msg
    let mut pr_msg = String::from(format!("- {branch}\n- Staging:"));
    for file in diff_files {
        pr_msg.push_str(&format!("\n  - {staging_pr_base}/{file}"));
    }

    pr_msg.push_str(&format!("\n- Build log: {build}"));
    pr_msg
}

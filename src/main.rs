use git2::Repository;
use std::collections::HashMap;
use std::io;
use std::process::Command;

const DEBUG: bool = true;

fn main() {
    let mut repo2staging: HashMap<&str, &str> = HashMap::default();
    repo2staging.insert(
        "cloud-docs",
        "https://docs-atlas-staging.mongodb.com/cloud-docs/docsworker-xlarge",
    );
    repo2staging.insert(
        "mms-docs",
        "https://docs-opsmanager-staging.mongodb.com/docsworker-xlarge",
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
        .arg("master")
        .output()
        .expect("failed to execute process");
    let diff = String::from_utf8(diff.stdout).expect("whoops");
    let diff_files: Vec<&str> = diff.split("\n").collect();
    let diff_files: Vec<&str> = diff_files
        .iter()
        .map(|s| s.replace("source/", "").strip_suffix(".txt").unwrap())
        .collect();

    // Get build log
    let build = "";
    if !DEBUG {
        let mut build = String::new();
        println!("Input build log link: ");
        io::stdin()
            .read_line(&mut build)
            .expect("Failed to read line");
    };

    // Build PR msg
    let mut pr_msg = String::from(format!("- {branch}\n- Staging:"));
    for file in diff_files {
        pr_msg.push_str(&format!("\n  - {staging_base}/{branch}/{file}"));
    }

    pr_msg.push_str(&format!("\n- Build log: {build}"));
    println!("{}", pr_msg);
}

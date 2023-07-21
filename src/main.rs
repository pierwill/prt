#[allow(dead_code)]
use git2::Repository;

const repo2staging: (&str, &str) = (
    "cloud-docs",
    "https://docs-atlas-staging.mongodb.com/cloud-docs/docsworker-xlarge/",
);

fn main() {
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let head = match repo.head() {
        Ok(head) => Some(head),
        Err(ref e)
            if e.code() == git2::ErrorCode::UnbornBranch
                || e.code() == git2::ErrorCode::NotFound =>
        {
            None
        }
        Err(e) => panic!(),
    };
    let head = head.as_ref().and_then(|h| h.shorthand());

    println!("{:?}{:?}", head, repo.workdir().unwrap());
}

# prt

`prt` is for the cloud docs team at MongoDB to help prepare PR messages.

Let's say you have a branch named `DOCSP-99999-hello`. (The `hello` suffix is fine to have.)

When you're done working, and you have a build log URL, you can run `prt '<build log URL>'`
(make sure to use quotes around the URL) and get a PR message that looks like this:

```
- DOCSP-38030
- Staging:
  - https://preview-mongodbpierwill.gatsbyjs.io/cloud-docs/DOCSP-38030-rm-compose/import
  - https://preview-mongodbpierwill.gatsbyjs.io/cloud-docs/DOCSP-38030-rm-compose/import/live-import-troubleshooting
  - https://preview-mongodbpierwill.gatsbyjs.io/cloud-docs/DOCSP-38030-rm-compose/legacy-migration
- Build log: https://workerpool-boxgs.mongodbstitch.com/pages/job.html?collName=queue&jobId=660c196a3a5920a2913be1b8
```

# Install

You will need to (install Rust)[https://www.rust-lang.org/tools/install].

1. Download this repo.
2. `cd prt/`
3. `cargo install --path .`

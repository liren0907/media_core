## Release Workflow (dev → main + tag)

This project follows a simple branch-based flow with semantic versioning tags.
All day-to-day work lands in `dev`, releases are cut from `main`, and every
release is tagged (e.g., `v0.1.0`).

### Branching model
- Feature/fix branches → pull requests into `dev`.
- `dev` is the integration/staging branch.
- `main` holds only releasable commits.

### Release prep (on `dev`)
1) Ensure code is ready: merge latest feature PRs into `dev`.
2) Bump crate versions to the target semver (e.g., `0.1.0`) in all crates you ship.
3) Update `CHANGELOG.md`:
   - Maintain an `Unreleased` section.
   - Move its entries under the new version heading (e.g., `## [0.1.0] - YYYY-MM-DD`).
   - Summarize notable changes (features, fixes, breaking changes).
4) Run full checks/tests (`cargo test --workspace`, fmt/clippy as desired).
5) Freeze `dev` until release is cut.

### Cut the release
1) Merge `dev` → `main` (fast-forward preferred to keep history linear).
2) Tag the release on `main` with an annotated tag:
   - `git tag -a v0.1.0 -m "Release v0.1.0"`
   - `git push origin main v0.1.0`
3) Optional: create a GitHub/GitLab release referencing the tag and
   include the changelog entry.

### Post-release (back on `dev`)
- Add a fresh `Unreleased` section at the top of `CHANGELOG.md`.
- Optionally bump versions to the next patch/minor pre-release if you want to signal ongoing development.

### Hotfixes
- Branch from `main`, apply fix, bump patch version, update changelog.
- Merge hotfix branch back to `main`, tag (e.g., `v0.1.1`), push tag.
- Merge `main` back into `dev` to keep branches in sync.

### Quick command reference
```bash
# prep (on dev)
cargo test --workspace
git status

# merge & tag (after fast-forwarding main)
git checkout main
git merge --ff-only dev
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin main v0.1.0
```


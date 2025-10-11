[![CI](https://github.com/lpenz/github-user-repos-status/actions/workflows/ci.yml/badge.svg)](https://github.com/lpenz/github-user-repos-status/actions/workflows/ci.yml)
[![coveralls](https://coveralls.io/repos/github/lpenz/github-user-repos-status/badge.svg?branch=main)](https://coveralls.io/github/lpenz/github-user-repos-status?branch=main)
[![dependency status](https://deps.rs/repo/github/lpenz/github-user-repos-status/status.svg)](https://deps.rs/repo/github/lpenz/github-user-repos-status)

# github-user-repos-status

*github-user-repos-status* create a github page with the status of all
of the user's repositories, including stars and README.md shields. The
page it generates for lpenz can be seen at
https://www.lpenz.org/github-user-repos-status/


## Usage

- Clone this repository
- Change the `lpenz` username to yours wherever you find it.
- In https://github.com/settings/tokens create a *personal access
  token*.
- In the new repository's Settings / Secrets / Actions create a
  `USER_REPOS_STATUS` secret containing the token created in the
  previous step.
- In the new repository's Settings / Actions / General, under
  "Workflow permissions", mark "Read and write permissions"
- In the new repository's Settings / Pages, under "Build and
  deployment" select the "Source" as "Deploy from a branch" and set
  the branch to "gh-pages".


There are certainly other ways to set this up, take a look at the
github actions used and adapt it to your needs.

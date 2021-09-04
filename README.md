# github-repo-normalized

This project aims to help ensuring you or your organizations repositories' settings and branch protection rules conform to the standard you have set up.

The standard will be specified as a json file (currently split up into two files) where values you have nulled means you do not care what the value of that is.

## Known issues

- "Default branch naming" of master and main might be interchangeable for some repositories, but for the branch protection rules it has to be strict. This could potentially be solved by having a parameter as part of the branch protection rule which says "create if it doesn't exists" or "update if it exists".
- All code is in the same file xD.
- The standard is currently split up into two files, it should probably be one file.
- There's no way to query an organization right now, might need to solve this with some god of war template magic.

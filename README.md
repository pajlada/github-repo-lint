# github-repo-normalizer

This project aims to help ensuring you or your organizations repositories' settings and branch protection rules conform to the standard you have set up.

The standard will be specified as a json file (currently split up into two files) where values you have nulled means you do not care what the value of that is.

## Known issues

- "Default branch naming" of master and main might be interchangeable for some repositories, but for the branch protection rules it has to be strict. This could potentially be solved by having a parameter as part of the branch protection rule which says "create if it doesn't exists" or "update if it exists".

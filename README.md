# github-repo-lint

This project aims to help ensuring you or your organizations repositories' settings and branch protection rules conform to the standard you have set up.

The standard is configured in a single json file (see `example.config.json`).

## Usage

Look for mismatching settings  
`./github-repo-lint --config <FILE> --user pajlada --organization pajlads`

Fix mismatching settings (works for most things, see labels in the settings below for where it doesn't work)  
`./github-repo-lint --config <FILE> --user pajlada --organization pajlads --fix`

## Update topics

You can ensure certain topics exist or don't exist in your repositories using the `topics` config key.

The `topics` config key expects a list of operations

### Operations

- `must_exist`  
   If the repository does not have the topic `name`, add it.

  ```json
  {
    "operation": "must_exist",
    "name": "hacktoberfest"
  }
  ```

- `must_not_exist`  
  If the repository has the topic `name`, remove it.

  ```json
  {
    "operation": "must_not_exist",
    "name": "hacktoberfest"
  }
  ```

- `rename`  
  If the repository has the topic `old_name`, remove it and add `name`.
  ```json
  {
    "operation": "rename",
    "old_name": "pajbot2020",
    "name": "pajbot2021"
  }
  ```

### Full example

Ensure `prod` topic exists, `hacktoberfest` topic does not exist, and if the `pajbot2020` topic exists, it's renamed to `pajbot2021`.

```json
{
  ...,
  "topics": [
    {
      "operation": "must_exist",
      "name": "prod"
    },
    {
      "operation": "must_not_exist",
      "name": "hacktoberfest"
    },
    {
      "operation": "rename",
      "old_name": "pajbot2020",
      "name": "pajbot2021"
    }
  ]
}
```

## Update settings

You can ensure certain repository settings are set to your desired value using the `settings` config key.

The `settings` config key expects an object, where the key is one of the valid keys listed below, and the value is a nullable bool.

### Value

- `null` = Leave the setting as is.
- `true` = Ensure the setting is enabled.
- `false` = Ensure the setting is disabled.

### Keys

- `allow_auto_merge`  
  Allow setting pull requests to merge automatically once all required reviews and status checks have passed.
- `allow_merge_commit`  
  Allow merge commit method to be used when merging pull requests.
- `allow_rebase_merge`  
  Allow rebase commit method to be used when merging pull requests.
- `allow_squash_merge`  
  Allow squash commit method to be used when merging pull requests.
- `has_issues`  
  Enable the issues feature in the repository.
- `has_projects`  
  Enable the projects feature in the repository.
- `has_wiki`  
  Enable the wiki feature in the repository.

### Full example

Ensure auto merging pull requests is allowed, and that only the squash pull request merge method is allowed.

```json
{
  ...,
  "settings": {
    "allow_auto_merge": true,
    "has_issues": null,
    "has_projects": null,
    "has_wiki": null,
    "allow_merge_commit": false,
    "allow_squash_merge": true,
    "allow_rebase_merge": false
  },
}
```

## Update branch protection rules

⚠ Not yet implemented ⚠

## Known issues

- "Default branch naming" of master and main might be interchangeable for some repositories, but for the branch protection rules it has to be strict. This could potentially be solved by having a parameter as part of the branch protection rule which says "create if it doesn't exists" or "update if it exists".

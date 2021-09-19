# github-repo-normalizer

This project aims to help ensuring you or your organizations repositories' settings and branch protection rules conform to the standard you have set up.

The standard will be specified as a json file (currently split up into two files) where values you have nulled means you do not care what the value of that is.

## Update topics

You can ensure certain topics exist or don't exist in your repositories using the `topics` config key.

The `topics` config key expects a list of operations

### Operations

- `must_exist`  
   If the repository does not have the topic `name`, add it.

  ```
  {
    "operation": "must_exist",
    "name": "hacktoberfest"
  }
  ```

- `must_not_exist`  
  If the repository has the topic `name`, remove it.

  ```
  {
    "operation": "must_not_exist",
    "name": "hacktoberfest"
  }
  ```

- `rename`  
  If the repository has the topic `old_name`, remove it and add `name`.
  ```
  {
    "operation": "rename",
    "old_name": "pajbot2020",
    "name": "pajbot2021"
  }
  ```

### Full example

Ensure `prod` topic exists, `hacktoberfest` topic does not exist, and if the `pajbot2020` topic exists, it's renamed to `pajbot2021`.

```
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

## Known issues

- "Default branch naming" of master and main might be interchangeable for some repositories, but for the branch protection rules it has to be strict. This could potentially be solved by having a parameter as part of the branch protection rule which says "create if it doesn't exists" or "update if it exists".

# Leaves
## General Flags
```
--last-n-days  <n>  -d
--last-n-weeks <n>  -w
--ignore-hidden     -i  DEFAULT
```

## Configuration
`.config/leaves/leaves.toml`:
```toml
path = "/persistent/old_roots" DEFAULT
```

## Restore
```sh
leaves restore ./file.txt
leaves r ./file.txt
```
Open a fuzzy finder with options for all roots that contain ./file.txt at that location, copy the picked one to the location 

```sh
leaves restore --automatic ./file.txt
leaves r -a ./file.txt
```
Copy the lastest found file with that name at that location to this location


## Query
```sh
leaves query file.txt
leaves q file.txt
```
Find files that match the file name in roots

```sh
leaves query --path ".*a\.txt"
leaves q -p ".*a\.txt"
```
Find files that match the regex path (root directory stripped)


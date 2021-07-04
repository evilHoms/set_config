# set_config

Script to switch specific prepared files in selected dirrectory.
For example to switch between few config files.

### How to

- Show all available configs
`./set_config`

- Switch to selected config
`./set_config dev`

- Path and pattern for files that could be swithced set in `settings.json` as well as folder and name of result file. It should be created by `settings.example.json` example.

### Example

```
{
    "CONFIG_PATH": "./configs",
    "CONFIG_PATTERN": ".config.json",
    "DEST_CONFIG_PATH": "./public",
    "DEST_CONFIG_NAME": "app.config.json"
}
```
```
|___ set_config
|___ configs
|      |___ dev.config.json
|      |___ prod.config.json
|
|___ public
         |___ app.config.json
```
In this case any of configs in `./configs` folder could replace `app.config.json` file in public folder via `set_config`.


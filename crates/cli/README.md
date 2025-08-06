# inkanim

![Cyberpunk 2077 version compatibility](https://img.shields.io/badge/Cyberpunk_2077-patch_2.3-yellow) ![WolvenKit](https://img.shields.io/badge/8.16.2-red?label=WolvenKit&color=red&link=https%3A%2F%2Fwiki.redmodding.org%2Fwolvenkit) ![WKitJsonVersion](https://img.shields.io/badge/0.0.9-yellow?label=WKitJsonVersion&link=https%3A%2F%2Fwiki.redmodding.org%2Fwolvenkit)



CLI commands to quickly introspect `.inkwidget` and corresponding `.inkanim` exported as JSON from [Wolvenkit](https://wiki.redmodding.org/wolvenkit/readme) when [modding](https://wiki.redmodding.org/home/) the game [Cyberpunk 2077](https://www.cyberpunk.net/).

## install

install with

```sh
cargo install inkanim
```

## use

available commands:

- list: quickly introspect and filters by interpolator type or widget indexes path

  ```sh
  inkanim list --help
  ```

- whois: quickly get widget names path from indexes path

  ```sh
  inkanim whois --help
  ```

- whereis: quickly get widget indexes path from names path

  ```sh
  inkanim whereis --help
  ```

### samples output

![screenshot: list](https://github.com/cyb3rpsych0s1s/inkanim/raw/main/screenshots/list.png)
![screenshot: whois](https://github.com/cyb3rpsych0s1s/inkanim/raw/main/screenshots/whois.png)
![screenshot: whereis](https://github.com/cyb3rpsych0s1s/inkanim/raw/main/screenshots/whereis.png)

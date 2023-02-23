# InkAnim

![build](https://github.com/cyb3rpsych0s1s/inkanim/actions/workflows/quality.yml/badge.svg) ![crates.io](https://img.shields.io/crates/v/cargo-inkanim.svg)

This little tool allows to quickly introspect `.inkwidget` and corresponding `.inkanim` exported as JSON from [Wolvenkit](https://wiki.redmodding.org/wolvenkit/readme) when [modding](https://wiki.redmodding.org/home/) the game [Cyberpunk 2077](https://www.cyberpunk.net/).

## why ?

Browsing in WolvenKit is fine when there's a couple of widgets and interpolators,
but it can quickly become tedious or close to impossible when there's hundreds of [inkanimInterpolator](https://nativedb.red4ext.com/inkanimInterpolator)s and deeply-nested [inkWidget](https://nativedb.red4ext.com/inkWidget)s.
>
> if you don't believe me, have a look in WolvenKit at `base\\gameplay\\gui\\quests\\q001\\q001_mission0_connect_to_girl.inkwidget` and `base\\gameplay\\gui\\quests\\q001\\q001_mission0_connect_to_girl_animations.inkanim` :wink:
> this is the anim for the biomonitor from the mission "The Rescue" : watch on [YouTube](https://youtu.be/J5ar3ynfcN4?t=404).

![screenshot: list](https://github.com/cyb3rpsych0s1s/inkanim/raw/main/screenshots/list.png)
![screenshot: whois](https://github.com/cyb3rpsych0s1s/inkanim/raw/main/screenshots/whois.png)
![screenshot: whereis](https://github.com/cyb3rpsych0s1s/inkanim/raw/main/screenshots/whereis.png)

## usage

install with

```sh
cargo install inkanim
```

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

## roadmap

This tool is in its early stage, so please open an [issue](https://github.com/cyb3rpsych0s1s/inkanim/issues) if you find any bug.

Feel free to come [discuss](https://github.com/cyb3rpsych0s1s/inkanim/discussions) any feature you feel is missing.

Contributions welcomed !

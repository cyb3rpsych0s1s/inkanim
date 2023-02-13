# Cargo InkAnim

![build](https://github.com/cyb3rpsych0s1s/inkanim-utils/actions/workflows/quality.yml/badge.svg)

This little tool allows to quickly introspect `.inkwidget` and corresponding `.inkanim` exported as JSON from [Wolvenkit](https://wiki.redmodding.org/wolvenkit/readme) when [modding](https://wiki.redmodding.org/home/) the game [Cyberpunk 2077](https://www.cyberpunk.net/).

## why ?

Browsing in WolvenKit is fine when there's a couple of widgets and interpolators,
but it can quickly become tedious or close to impossible when there's hundreds of [inkanimInterpolator](https://nativedb.red4ext.com/inkanimInterpolator)s and deeply-nested [inkWidget](https://nativedb.red4ext.com/inkWidget)s.
>
> if you don't believe me, have a look in WolvenKit at `base\\gameplay\\gui\\quests\\q001\\q001_mission0_connect_to_girl.inkwidget` and `base\\gameplay\\gui\\quests\\q001\\q001_mission0_connect_to_girl_animations.inkanim` :wink:
> this is the anim for the biomonitor from the mission "The Rescue" : watch on [YouTube](https://youtu.be/J5ar3ynfcN4?t=404).

![screenshot: list](https://github.com/cyb3rpsych0s1s/inkanim-utils/raw/main/screenshots/list.png)
![screenshot: whois](https://github.com/cyb3rpsych0s1s/inkanim-utils/raw/main/screenshots/whois.png)
![screenshot: whereis](https://github.com/cyb3rpsych0s1s/inkanim-utils/raw/main/screenshots/whereis.png)

## usage

install with

```sh
cargo install cargo-inkanim
```

available commands:

- list: quickly introspect and filters by interpolator type or widget indexes path

  ```sh
  cargo-inkanim list --help
  ```

- whois: quickly get widget names path from indexes path

  ```sh
  cargo-inkanim whois --help
  ```

- whereis: quickly get widget indexes path from names path

  ```sh
  cargo-inkanim whereis --help
  ```

## roadmap

This tool doesn't support all the widgets types yet, so please open an [issue](https://github.com/cyb3rpsych0s1s/inkanim-utils/issues) if this is blocking for you.

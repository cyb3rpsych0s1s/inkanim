# InkAnim

![build](https://github.com/cyb3rpsych0s1s/inkanim/actions/workflows/quality.yml/badge.svg) [![crates.io](https://img.shields.io/crates/v/inkanim.svg)](https://crates.io/crates/inkanim) ![WolvenKit](https://img.shields.io/badge/8.13.0-red?label=WolvenKit&color=red&link=https%3A%2F%2Fwiki.redmodding.org%2Fwolvenkit) ![WKitJsonVersion](https://img.shields.io/badge/0.0.8-yellow?label=WKitJsonVersion&link=https%3A%2F%2Fwiki.redmodding.org%2Fwolvenkit)



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

## development

Quickly try out methods with:

```sh
cargo run list --path '1.3.0.0.6' --type progress --widget ./inkwidget_connect_to_girl.json
```

```sh
cargo run whois --path '1.3.0.0.6' --widget ./inkwidget_connect_to_girl.json
```

```sh
cargo run whereis --path "main_canvas.Booting_Info_Critica_Mask_Canvas.Booting_Info_Critical_Canvas.Booting_Screen.BOOTING_PROGRESS_Text" --widget ./inkwidget_connect_to_girl.json
```

## roadmap

This tool is in its early stage, so please open an [issue](https://github.com/cyb3rpsych0s1s/inkanim/issues) if you find any bug.

Feel free to come [discuss](https://github.com/cyb3rpsych0s1s/inkanim/discussions) any feature you feel is missing.

Contributions welcomed !

set windows-shell := ["pwsh", "-NoLogo", "-NoProfile", "-Command"]

_default:
  inkanim --help

# quickly test all commands
demo:
  cargo run list \
    --path '1.3.0.0.6' \
    --type progress \
    --widget ./inkwidget_connect_to_girl.json
  cargo run whois \
    --path '1.3.0.0.6' \
    --widget ./inkwidget_connect_to_girl.json
  cargo run whereis \
    --path "main_canvas.Booting_Info_Critica_Mask_Canvas.Booting_Info_Critical_Canvas.Booting_Screen.BOOTING_PROGRESS_Text" \
    --widget ./inkwidget_connect_to_girl.json
  cargo run tree \
    --widget ./inkwidget_connect_to_girl.json
  cargo run show \
    --path "main_canvas.Booting_Info_Critica_Mask_Canvas.Booting_Info_Critical_Canvas.Info_Screen.Info_MainScreen_Mask_Canvas.Info_MainScreen_Canvas.Critical_Screen_Text_Canvas.inkVerticalPanelWidget7.inkHorizontalPanelWidget2.Critical_Vertical_Warning.warning_Flex1.warning1" \
    --widget ./inkwidget_connect_to_girl.json

# 🎨 format code
format:
  @cargo fmt --all

# 🎨 lint code
@lint:
  cargo clippy --fix --allow-dirty --allow-staged --workspace
  cargo fix --allow-dirty --allow-staged --workspace
  just format

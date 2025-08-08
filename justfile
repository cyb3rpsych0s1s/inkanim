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

# 🎨 format code
format:
  @cargo fmt

# 🎨 lint code
@lint:
  cargo clippy --fix --allow-dirty --allow-staged
  cargo fix --allow-dirty --allow-staged
  just format

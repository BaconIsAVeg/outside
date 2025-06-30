Output demo.gif
Output demo.mp4
Output demo.webm

Set Shell "zsh"
Set FontSize 16
Set FontFamily "MonaspiceKr Nerd Font Mono"
Set Width 1100
Set Height 600
Set Padding 10
Set WindowBar Colorful
Set TypingSpeed 65ms
Set Framerate 30
Set CursorBlink true
Set Margin 20
Set MarginFill "#406080"
Set BorderRadius 10

# Setup
# Hide
# Do stuff
# Enter Wait
# Show

Sleep 500ms

Type "outside -h"
Sleep 500ms Enter Wait Sleep 2s

Enter
Type "outside -o detailed"
Sleep 500ms Enter Wait Sleep 2s

Enter
Type "outside -o detailed -u imperial -l 'New York, US'"
Sleep 500ms Enter Wait Sleep 2s

Enter
Type "outside -o json | jq | head -20"
Sleep 500ms Enter Wait Sleep 2s

Enter
Type "outside -o waybar -l 'Toronto, CA' -u metric | jq"
Sleep 500ms Enter Wait Sleep 2s

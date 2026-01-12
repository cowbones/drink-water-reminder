# Drink Water Reminder

Simple popup that tells you to take a drink of water. Only really supported in
Arch. Made public for fun.

## Install

```bash
makepkg -si
systemctl --user enable --now drink-water-reminder.service
```

## Uninstall

```bash
systemctl --user disable --now drink-water-reminder.service
sudo pacman -R drink-water-reminder
```

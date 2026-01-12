# Drink water notifier

Simple popup that tells you to take a drink of water. Only really supported in
Arch. Made public for fun.

## install

```bash
makepkg -si
systemctl --user enable --now drink-water-reminder.service
```

## uninstall

```bash
systemctl --user disable --now drink-water-reminder.service
sudo pacman -R drink-water-reminder
```

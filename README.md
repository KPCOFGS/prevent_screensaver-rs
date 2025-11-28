# Prevent Screensaver

A simple tool that keeps your computer awake by moving the mouse at regular intervals.

## What You Need to Install First

**On Ubuntu/Debian:**
```bash
sudo apt install libx11-dev libxtst-dev pkg-config
```

**On Fedora/RHEL/CentOS:**
```bash
sudo dnf install libx11-devel libXtst-devel pkg-config
```

**On Arch Linux:**
```bash
sudo pacman -S libx11 libxtst pkg-config
```

## How to Use

1. **Build the program:**
   ```bash
   cargo build --release
   ```

2. **Run it:**
   ```bash
   ./target/release/prevent_screensaver 30.0
   ```
   (Replace `30.0` with how many seconds between mouse movements)

3. **To stop:** Press `Ctrl+C`

The mouse will move slightly every 30 seconds (or whatever interval you choose) to prevent your screensaver from starting.

## Example
```bash
# Move every minute
./target/release/prevent_screensaver 60.0

# Move every 2 minutes  
./target/release/prevent_screensaver 120.0
```

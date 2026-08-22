Currently an extremely simple but fast desktop audio player. Made with Tauri, Svelte, and Rust.
Uses Rodio for audio playback, Lofty for metadata, and Base64 for artwork display.

## Current Features:
- Artwork display
- Audio selection
- Play/pause
- Volume
- Sample rate & bit depth display

## Planned Features:
- Seek bar
- Playlists
- Queues
- Settings
- Full metadata display using Lofty

```bash
git clone https://github.com/datamothd/klight.git
cd klight
npm install
npm run tauri dev

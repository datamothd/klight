Currently an extremely simple, but fast desktop audio player made with Tauri, Svelte, and Rust. <br>
Uses Rodio for audio playback, Lofty for metadata, and Base64 for image re-encoding.

## Current Features:
- Artwork display
- Audio selection
- Play/pause
- Volume
- Sample rate & bit depth display
- Seek bar

## Planned Features:
- Playlists
- Queues
- Settings
- Full metadata display using Lofty

```bash
git clone https://github.com/datamothd/klight.git
cd klight
npm install
npm run tauri dev

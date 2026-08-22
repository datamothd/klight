<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";

  import defaultArtwork from "../klight.png";

  let artwork: string | null = null;
  let isPaused = true;
  let currentlyPlaying = "None";
  let currentVolume = 100;
  let sampleRate: number | null = null;
  let bitDepth: number | null = null;

  async function audioSelect() {
    const file = await open({
      multiple: false,
      directory: false,
    });
    if (!file || Array.isArray(file)) {
      return;
    }
    await invoke("play_audio", { path: file });
    artwork = await invoke<string | null>("get_artwork", { path: file });
    sampleRate = await invoke<number | null>("get_sample_rate", { path: file });
    bitDepth = await invoke<number | null>("get_bit_depth", { path: file });

    currentlyPlaying = file;
    isPaused = false;
  }
  async function togglePause() {
    isPaused = await invoke<boolean>('toggle_pause');
  }
  async function setVolume(event: Event) {
    currentVolume = Number((event.currentTarget as HTMLInputElement).value);
    const volume = currentVolume / 100;
    await invoke("set_volume", { volume });
  }
</script>

<main>
  <p>{isPaused ? "Paused" : "Playing"}: {currentlyPlaying || "None"} {#if sampleRate && bitDepth} @ {sampleRate} Hz / {bitDepth} bits{/if}</p>
  {#if artwork}
    <img class="artwork" src={artwork} alt="Album Artwork" />
  {:else}
    <img class="artwork" src={defaultArtwork} alt="Default Artwork" />
  {/if}

  <div class="row"><button onclick={audioSelect}>Select Audio</button></div>
  <div class="row"><button onclick={togglePause}>Toggle Pause</button></div>
  <p>Volume: {currentVolume}</p><input type="range" min="0" max="100" value={currentVolume} oninput={setVolume}>
</main>

<style>
  .artwork {
    image-rendering: pixelated;
    width: 300px;
    height: 300px;
  }
</style>
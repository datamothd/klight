<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";

  import '@fontsource/montserrat/400.css';

  import defaultArtwork from "../klight.png";
  import defaultArtworki from "../klight_i.png";

  let isPaused = true;
  let isSeeking = false;
  let artwork: string | null = null;
  let sampleRate: number | null = null;
  let bitDepth: number | null = null;
  let fileName = '';
  let currentVolume = 100;
  let currentPosition = 0;
  let sliderPosition = 0;
  let duration = 0;
  
  const formatTime = (seconds: number) => {
    const totalSeconds = Math.max(0, Math.floor(seconds));
    const minutes = Math.floor(totalSeconds / 60);
    const remainingSeconds = totalSeconds % 60;

    return `${String(minutes).padStart(2, '0')}:${String(remainingSeconds).padStart(2, '0')}`;
  };

  const getSeekBarWidth = (trackDuration: number) =>
    Math.min(600, Math.max(300, 300 + trackDuration / 2));

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

    fileName = file.split(/[\\/]/).pop()?.replace(/\.[^/.]+$/, '') ?? '';

    const [position, trackDuration] = await invoke<[number, number | null]>("get_playback_info");
    currentPosition = position;
    sliderPosition = position;
    duration = trackDuration ?? 0;
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
  async function seekAudio(position: number) {
    await invoke('seek_audio', {
      position
    });
    currentPosition = position;
  }
  async function handleSeekInput(event: Event) {
    isSeeking = true;
    sliderPosition = Number((event.currentTarget as HTMLInputElement).value);
  }
  async function finishSeeking() {
    await seekAudio(sliderPosition);
    if (isPaused) {
      isPaused = await invoke<boolean>('toggle_pause');
    }
    isSeeking = false;
  }

  onMount(() => {
    const updatePlaybackInfo = async () => {
      if (isSeeking) return;

      const [position, trackDuration] = await invoke<[number, number | null]>("get_playback_info");
      currentPosition = position;
      sliderPosition = position;
      duration = trackDuration ?? 0;
      if (duration > 0 && position >= duration - 0.25) {
        isPaused = true;
      }
    };
    const interval = window.setInterval(updatePlaybackInfo, 250);
    return () => window.clearInterval(interval);
  });
</script>

<main>
  <p class="status">{isPaused ? "Paused" : "Playing"}: {fileName || "None"} <span class="green">{#if sampleRate && bitDepth} @ {sampleRate} Hz / {bitDepth} bits{/if}</span></p>

  <div class="artworkFrame">
    {#if artwork}
      <img class="artwork" src={artwork} alt="Album Artwork" />
    {:else}
      <picture>
        <source
          media="(prefers-color-scheme: light)"
          srcset={defaultArtworki}
        />
        <img class="artwork" src={defaultArtwork} alt="Default Artwork" />
      </picture>
    {/if}
  </div>

  <div class="row"><button onclick={audioSelect}>Select Audio</button></div>
  <div class="row"><button onclick={togglePause}>Toggle Pause</button></div>
  <input
    class="seekBar"
    type="range"
    min="0"
    max={duration}
    step="0.1"
    value={isSeeking ? sliderPosition : currentPosition}
    oninput={handleSeekInput}
    onchange={finishSeeking}
    style={`width: min(90vw, ${getSeekBarWidth(duration)}px)`}
  />
  <p>{formatTime(currentPosition)} / {formatTime(duration)}</p>
  <p>Volume: {currentVolume}</p>
  <input
    class="volumeSlider"
    type="range"
    min="0" 
    max="100"
    value={currentVolume} 
    oninput={setVolume}
  />
</main>
<style>
  :root {
    background-color: #000000;
    color: #FFFFFF;
    font-family: "Montserrat", sans-serif;
    user-select: none;
  }
  main {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    width: 100%;
    box-sizing: border-box;
  }
  .green {
    color: rgb(0, 189, 0);
  }
  .status {
    min-height: 1.5em;
  }
  .artworkFrame {
    width: 300px;
    height: 300px;
    margin-bottom: 7px;
  }
  .volumeSlider {
    accent-color: #FFFFFF;
  }
  .seekBar {
    margin-top: 10px;
    max-width: 90vw;
    accent-color: #FFFFFF
  }
  .artwork {
    image-rendering: pixelated;
    width: 300px;
    height: 300px;
    margin-bottom: 7px;
  }
  @media (prefers-color-scheme: light) {
    :root {
      background-color: #FFFFFF;
      color: #000000;
    }
    .volumeSlider {
      accent-color: #000000;
    }
    .seekBar {
      accent-color: #000000;
    }
  }
</style>
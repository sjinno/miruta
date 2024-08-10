<script lang="ts">
  import type { AnimeDetail as Anime } from "../bindings/AnimeDetail";
  import { showAnimeDetail } from "../store";
  import AnimeDetail from "./AnimeDetail.svelte";

  export let anime: Anime;
</script>

{#if $showAnimeDetail}
  <div class="modal-overlay" on:click={showAnimeDetail.hide} aria-hidden="true">
    <div class="modal-content" on:click|stopPropagation aria-hidden="true">
      <AnimeDetail {anime} />
    </div>
  </div>
{/if}

<style lang="scss">
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(#000, 0.32);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: var(--m-max-z);
  }

  .modal-content {
    background-color: #000;
    color: #dadada;
    border-radius: 10px;
    overflow: auto;
    z-index: calc(var(--m-max-z) + 1);
    width: 96vw;
    max-width: 64rem;
    height: calc(64rem * 36 / 24);
    max-height: 88vh;
  }
</style>

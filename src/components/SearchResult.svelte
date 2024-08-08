<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import type { AnimeDetail } from "../bindings/AnimeDetail";
  import ActionButton from "./ActionButton.svelte";
  import { onMount } from "svelte";
  import { searchResults, watchWatchedList } from "../store";

  export let anime: AnimeDetail;
  export let handleResultClick: (a: AnimeDetail) => void;

  let toWatch = anime.to_watch;
  let watched = anime.watched;

  $: {
    toWatch =
      $watchWatchedList.find((a) => a.id === anime.id)?.to_watch ?? false;
    watched =
      $watchWatchedList.find((a) => a.id === anime.id)?.watched ?? false;
  }

  async function handleUpsertWatchlistClick() {
    await invoke("upsert_anime", { anime, props: { to_watch: !toWatch } });
    watchWatchedList.updateList();
    searchResults.update((results) => {
      const clone = { ...results };
      const index = clone.data.findIndex((a) => a.id === anime.id);
      if (index !== -1) {
        clone.data[index].to_watch = !toWatch;
      }
      return clone;
    });
    toWatch = !toWatch;
  }

  async function handleUpsertWatchedClick() {
    const date = new Date();
    console.log("shohei - new Date()", {
      m: date.getMonth(),
      y: date.getFullYear(),
    });
    await invoke("upsert_anime", { anime, props: { watched: !watched } });
    watchWatchedList.updateList();
    searchResults.update((results) => {
      const clone = { ...results };
      const index = clone.data.findIndex((a) => a.id === anime.id);
      if (index !== -1) {
        clone.data[index].watched = !watched;
      }
      return clone;
    });
    watched = !watched;
  }

  onMount(() => {
    return () => {
      // console.log("shohei - anime", anime);
    };
  });
</script>

<div
  class="search-result card"
  on:click={() => handleResultClick(anime)}
  aria-hidden="true"
>
  <img
    src={anime.image_url}
    alt={anime.title_english ?? anime.title}
    class="card__img"
  />
  <div class="card__detail">
    <div class="card__detail__flex1">
      <h3 class="card__detail__title">
        {anime.title_english ?? anime.title}
      </h3>
      <div class="card__detail__stat">
        <div>
          {anime.score?.toFixed(2) ?? "NaN"}/10 (scored by {anime.scored_by ??
            "N/A"})
        </div>
        <div>
          {anime.year ?? "N/A"} ・ {anime.rating?.split(" ")[0] ?? "N/A"}
        </div>
      </div>
    </div>
    <div class="card__detail__buttons-container">
      <ActionButton
        className={toWatch ? "watchlist--yes" : "watchlist--no"}
        label={toWatch ? "Remove from watchlist" : "Add to watchlist"}
        handleClick={handleUpsertWatchlistClick}
      />
      <ActionButton
        className={watched ? "watched--yes" : "watched--no"}
        label={watched ? "Mark as unwatched" : "Mark as watched"}
        handleClick={handleUpsertWatchedClick}
      />
    </div>
  </div>
</div>

<style lang="scss">
  .card {
    position: relative;
    width: 100%;
    height: 100%;
    background-color: var(--m-color-gray);
    border-radius: 10px;
    box-shadow: rgba(99, 99, 99, 0.2) 0px 2px 8px 0px;

    &:hover {
      cursor: pointer;
      transform: scale(1.01);
      box-shadow: rgba(0, 0, 0, 0.35) 0px 5px 15px;
    }

    &__img {
      position: absolute;
      top: 0;
      left: 0;
      height: 100%;
      aspect-ratio: var(--m-aspect-ratio-w);
      border-radius: 10px 0 0 10px;
    }

    &__detail {
      position: absolute;
      top: 0;
      left: var(--m-search-result-img-width);
      width: calc(100% - var(--m-search-result-img-width));
      height: 100%;
      padding: 0.5rem 1rem;
      display: flex;
      flex-direction: column;

      &__flex1 {
        height: 52%;
      }

      &__title {
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
        font-size: var(--m-fs-sans-xxs);
        font-weight: var(--m-fw-sans-b);
        margin-top: -0.25rem;
      }

      &__stat {
        margin-top: 0.5rem;

        div {
          font-size: var(--m-fs-sans-xxxs);
          font-weight: var(--m-fw-sans-xxl);
        }

        div:first-child {
          margin-bottom: 0.15rem;
        }
      }

      &__buttons-container {
        height: 48%;
        display: flex;
        gap: 1rem;
      }
    }
  }
</style>

<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import type { AnimeDetail as Anime } from "../bindings/AnimeDetail";
  import ActionButton from "./ActionButton.svelte";
  import { searchResults, watchWatchedList } from "../store";

  export let anime: Anime;
  export let showAnimeModal: (a: Anime) => void;

  const THRESHOLD = 42;

  const animeName_ = anime.title_english ?? anime.title;
  const animeName =
    animeName_.length <= THRESHOLD
      ? animeName_
      : animeName_.substring(0, THRESHOLD) + "...";

  let toWatch = anime.to_watch;
  let watched = anime.watched;

  $: {
    toWatch =
      $watchWatchedList.find((a) => a.id === anime.id)?.to_watch ?? false;
    watched =
      $watchWatchedList.find((a) => a.id === anime.id)?.watched ?? false;
  }

  async function handleUpsertWatchlistClick() {
    try {
      await invoke("upsert_anime", { anime, props: { to_watch: !toWatch } });
      watchWatchedList.updateList();
      if ($searchResults && $searchResults.data.length) {
        searchResults.update((results) => {
          const clone = { ...results, data: [...results.data] };
          const index = clone.data.findIndex((a) => a.id === anime.id);
          if (index !== -1) {
            clone.data[index] = { ...clone.data[index], to_watch: !toWatch };
          }
          return clone;
        });
        toWatch = !toWatch;
      }
    } catch (error) {
      console.error("Error updating watchlist:", error);
    }
  }

  async function handleUpsertWatchedClick() {
    try {
      const date = new Date();
      console.log("shohei - new Date()", {
        m: date.getMonth(),
        y: date.getFullYear(),
      });
      await invoke("upsert_anime", { anime, props: { watched: !watched } });
      watchWatchedList.updateList();
      if ($searchResults && $searchResults.data.length) {
        searchResults.update((results) => {
          const clone = { ...results, data: [...results.data] };
          const index = clone.data.findIndex((a) => a.id === anime.id);
          if (index !== -1) {
            clone.data[index] = { ...clone.data[index], watched: !watched };
          }
          return clone;
        });
        watched = !watched;
      }
    } catch (error) {
      console.error("Error updating watched list:", error);
    }
  }

  async function handleDeleteClick() {
    try {
      console.log("shohei - deleting anime", anime);
      await invoke("delete_anime", { id: anime.id });
      watchWatchedList.updateList();
      if ($searchResults && $searchResults.data.length) {
        searchResults.update((results) => {
          const clone = { ...results, data: [...results.data] };
          const index = clone.data.findIndex((a) => a.id === anime.id);
          if (index !== -1) {
            clone.data[index] = {
              ...clone.data[index],
              to_watch: false,
              watched: false,
            };
          }
          return clone;
        });
      }
    } catch (error) {
      console.error("Error deleting anime:", error);
    }
  }
</script>

<div class="anime">
  <img
    src={anime.large_image_url}
    alt={anime.title_english ?? anime.title}
    class="anime__img"
  />
  <div class="anime__detail">
    <div
      class="anime__detail__title"
      on:click={() => showAnimeModal(anime)}
      aria-hidden="true"
    >
      <h3 class="anime__detail__title">{anime.title_english ?? anime.title}</h3>
    </div>
    <div class="anime__detail__buttons-container">
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
      <ActionButton
        className="delete"
        label="Delete"
        handleClick={handleDeleteClick}
      />
    </div>
  </div>
  <div class="anime__name">
    <h3 class="anime__title">{animeName}</h3>
  </div>
</div>

<style lang="scss">
  .anime {
    position: relative;
    z-index: var(--m-max-z);

    // fields to be changed for different screen sizes
    height: calc(123vw + 4.75rem);
    aspect-ratio: var(--m-aspect-ratio-n);
    background-color: var(--m-color-blue);

    &__img {
      position: absolute;
      left: 0;
      top: 0;
      z-index: -1;

      // fields to be changed for different screen sizes
      height: 123vw;
      width: 100%;
    }

    &__title {
      font-size: var(--m-fs-sans-s);
      font-weight: var(--m-fw-sans-n);
      margin-top: 0.35rem;
    }

    &__name {
      position: absolute;
      left: 0;
      width: 100%;

      // fields to be changed for different screen sizes
      top: 123vw;
      height: 4.75rem;
    }

    &__detail {
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      opacity: 0;
      color: #dadada;
      padding: 1rem;
      display: flex;
      flex-direction: column;
      z-index: 1000;

      &:hover {
        opacity: 1;
        background-color: rgba(0, 0, 0, 0.8);
        transition:
          opacity 0.1s ease-in,
          background-color 0.1s ease-in;
      }

      &__title {
        width: 100%;
        height: 58%;
        font-size: var(--m-fs-sans-m);
        font-weight: var(--m-fw-sans-b);
      }

      &__buttons-container {
        width: 100%;
        height: 42%;
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
        justify-content: end;
      }
    }
  }

  // css media query template
  @media (min-width: 320px) and (max-width: 480px) {
    .anime {
      height: 32rem;

      &__img {
        height: 28rem;
      }

      &__name {
        top: 28rem;
        height: 4rem;
      }
    }
  }
</style>

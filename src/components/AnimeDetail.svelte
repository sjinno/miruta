<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import type { AnimeDetail } from "../bindings/AnimeDetail";
  import { watchWatchedList, searchResults, showAnimeDetail } from "../store";
  import ActionButton from "./ActionButton.svelte";

  export let anime: AnimeDetail;

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
</script>

{#if anime}
  <div class="anime-modal">
    <img
      src={anime.large_image_url}
      alt={anime.title_english ?? anime.title}
      class="anime-modal__img"
    />
    <div class="anime-modal__detail">
      <h2 class="anime-modal__detail__title">
        {anime.title_english ?? anime.title}
      </h2>
      <div class="anime-modal__detail__meta">
        <div>
          <span class="bold">Rank:</span>
          {anime.rank ?? "Unknown"}
        </div>
        <div>
          <span class="bold">Score:</span>
          {anime.score ?? "Unknown"}
        </div>
        <div>
          <span class="bold">Rating:</span>
          {anime.rating?.split(" ")[0]}
        </div>
        <div class="anime-modal__detail__synopsys">
          <span class="bold">Synopsys:</span>
          {anime.synopsis}
        </div>
      </div>
      <div class="anime-modal__buttons-container">
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
{/if}

<style lang="scss">
  .anime-modal {
    position: relative;
    width: 100%;
    height: 100%;

    &__img {
      position: absolute;
      left: 50%;
      top: 50%;
      transform: translate(-50%, -50%);
      width: 100%;
      height: 100%;
      opacity: 0.2;
      z-index: -1;
      object-fit: contain;
    }

    &__detail {
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      padding: var(--m-anime-detail-padding)
        calc(var(--m-anime-detail-padding) * 1.25);
      overflow: auto;

      &__title {
        font-size: var(--m-fs-sans-m);
        font-weight: var(--m-fw-sans-xxb);
        margin-bottom: 1.5rem;
      }

      &__meta {
        font-size: var(--m-fs-sans-s);
        letter-spacing: 0.6px;
        line-height: 1.3;
        margin-bottom: 1rem;

        div {
          margin-bottom: 0.25rem;
        }

        .bold {
          font-weight: var(--m-fw-sans-xb);
          font-size: var(--m-fs-sans-s);
        }
      }
    }

    &__buttons-container {
      display: flex;
      gap: 1rem;
      margin-block: 3rem 2rem;
    }
  }
</style>

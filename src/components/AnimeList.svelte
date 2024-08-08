<script lang="ts">
  import { onMount } from "svelte";
  import type { AnimeDetail } from "../bindings/AnimeDetail";
  import { showAnimeDetail, watchWatchedList } from "../store";
  import SearchResult from "./SearchResult.svelte";
  import Modal from "./Modal.svelte";
  import type { ListType } from "../types";

  export let listType: ListType;
  let anime: AnimeDetail;
  const isWatchlist = listType === "watchlist";

  function showAnimeModal(a: AnimeDetail) {
    showAnimeDetail.show();
    anime = a;
  }

  function handleCloseModal(e: KeyboardEvent) {
    if (e.key === "Escape") showAnimeDetail.hide();
  }

  onMount(() => {
    window.addEventListener("keydown", handleCloseModal);
    return () => {
      window.removeEventListener("keydown", handleCloseModal);
    };
  });
</script>

{#if $watchWatchedList.every((a) => (isWatchlist ? !a.to_watch : !a.watched))}
  <p class="empty">
    Your {isWatchlist ? "watchlist" : "watched list"} is empty.
  </p>
{:else}
  <div class="anime-items">
    {#each $watchWatchedList as a (a.id)}
      {#if isWatchlist ? a.to_watch : a.watched}
        <div class="anime-items__item">
          <SearchResult anime={a} handleResultClick={showAnimeModal} />
        </div>
      {/if}
    {/each}
  </div>
  {#if $showAnimeDetail}
    <Modal {anime} />
  {/if}
{/if}

<style lang="scss">
  .empty {
    width: 95%;
    margin: 6rem auto;
    text-align: center;
    font-size: var(--m-fs-sans-l);
    font-weight: var(--m-fw-sans-xxxl);
  }

  .anime-items {
    width: 31.5rem;
    margin: auto;
    margin-block: 5.25rem;
    display: flex;
    flex-wrap: wrap;
    row-gap: 1.5rem;
    column-gap: 2rem;

    &__item {
      width: 31.5rem;
      height: 12rem;
    }
  }

  // css media query template
  @media (min-width: 650px) and (max-width: 984px) {
    .anime-items {
      width: 65rem;
    }
  }

  @media (min-width: 985px) and (max-width: 1319px) {
    .anime-items {
      width: 98.5rem;
    }
  }

  @media (min-width: 1320px) and (max-width: 1654px) {
    .anime-items {
      width: 132rem;
    }
  }

  @media (min-width: 1655px) and (max-width: 1989px) {
    .anime-items {
      width: 165.5rem;
    }
  }
</style>

<script lang="ts">
  import type { AnimeDetail as Anime } from "../bindings/AnimeDetail";
  import { searchResults, showAnimeDetail } from "../store";
  import Modal from "./Modal.svelte";
  import Pagination from "./Pagination.svelte";
  import SearchResult from "./SearchResult.svelte";

  let anime: Anime;

  function handleResultClick(a: Anime) {
    showAnimeDetail.show();
    anime = a;
  }
</script>

{#if $searchResults && $searchResults.data.length}
  <Pagination />
  <div class="search-results">
    {#each $searchResults.data as anime (anime.id)}
      <div class="search-results__item">
        <SearchResult {anime} {handleResultClick} />
      </div>
    {/each}
  </div>
  <Pagination />
{/if}

{#if $showAnimeDetail}
  <Modal {anime} />
{/if}

<style lang="scss">
  .search-results {
    width: 31.5rem;
    margin: auto;
    display: flex;
    flex-wrap: wrap;
    column-gap: 2rem;
    row-gap: 1.5rem;

    &__item {
      width: 31.5rem;
      height: 12rem;
    }
  }

  // css media query template
  @media (min-width: 320px) and (max-width: 649px) {
    .search-results {
      width: 85%;

      &__item {
        flex-grow: 1;
      }
    }
  }

  @media (min-width: 650px) and (max-width: 984px) {
    .search-results {
      width: 65rem;
    }
  }

  @media (min-width: 985px) and (max-width: 1319px) {
    .search-results {
      width: 98.5rem;
    }
  }

  @media (min-width: 1320px) and (max-width: 1654px) {
    .search-results {
      width: 132rem;
    }
  }

  @media (min-width: 1655px) and (max-width: 1989px) {
    .search-results {
      width: 165.5rem;
    }
  }
</style>

<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { currentPage, searchResults } from "../store";
  import type { AnimeList } from "../bindings/AnimeList";

  let query: string;
  let lastVisiblePage: number;

  $: {
    query = $searchResults.query;
    lastVisiblePage = $searchResults.lastVisiblePage;
  }

  async function handlePageChange(page: number) {
    currentPage.set(page);
    const animeList: AnimeList = await invoke("query_anime", {
      name: query,
      page: $currentPage,
    });
    lastVisiblePage = animeList.pagination.last_visible_page;
    searchResults.set({
      query,
      data: animeList.data,
      lastVisiblePage,
    });
  }
</script>

{#if lastVisiblePage > 1}
  <div class="pagination">
    {#each Array.from({ length: lastVisiblePage }) as _, i}
      <button
        class="pagination__button"
        disabled={$currentPage === i + 1}
        on:click={() => handlePageChange(i + 1)}>{i + 1}</button
      >
    {/each}
  </div>
{/if}

<style lang="scss">
  .pagination {
    display: flex;
    justify-content: center;
    gap: 2rem;
    margin-block: 4rem;

    &__button {
      border: none;
      background-color: var(--m-color-white);
      font-size: var(--m-fs-sans-xs);
      font-weight: var(--m-fw-sans-xb);
      cursor: pointer;

      &:disabled {
        cursor: default;
      }
    }
  }
</style>

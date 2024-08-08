<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { currentPage, searchResults, watchWatchedList } from "../store";
  import type { AnimeList } from "../bindings/AnimeList";
  import type { AnimeDetail } from "../bindings/AnimeDetail";

  let data: AnimeDetail[];
  let lastVisiblePage: number;

  async function handleSearchSubmit() {
    currentPage.set(1);
    const animeList: AnimeList = await invoke("query_anime", {
      name: $searchResults.query,
      page: $currentPage,
    });
    data = animeList.data;
    lastVisiblePage = animeList.pagination.last_visible_page;
    for (const anime of $watchWatchedList) {
      const index = data.findIndex((a) => a.id === anime.id);
      if (index !== -1) {
        data[index].to_watch = true;
      }
    }
    searchResults.set({ query: $searchResults.query, data, lastVisiblePage });
  }

  function handleResetClick() {
    searchResults.set({ query: "", data: [], lastVisiblePage: 0 });
  }
</script>

<form action="" method="get" class="search-form" on:submit={handleSearchSubmit}>
  <input
    type="text"
    name="query"
    class="search-form__input"
    autocapitalize="off"
    spellcheck="false"
    bind:value={$searchResults.query}
    placeholder="Search..."
    required
  />
  <div class="search-form__buttons-container">
    <button
      type="submit"
      class="button-12 search-form__button search-form__button--submit"
      >Search</button
    >
    {#if $searchResults && $searchResults.data.length}
      <button
        on:click={handleResetClick}
        class="button-12 search-form__button search-form__button--reset"
        >Reset</button
      >
    {/if}
  </div>
</form>

<style lang="scss">
  .search-form {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin-block: 2rem;

    &__input {
      font-size: var(--m-fs-sans-s);
      margin-block: 2rem;
      width: 76%;
      max-width: 58rem;
      padding: 0.5rem 1rem;
    }

    &__buttons-container {
      display: flex;
      gap: 2rem;
    }

    &__button {
      font-size: var(--m-fs-sans-xs);
      padding: 0.5rem 1.5rem;
      border-radius: 0.75rem;
      background-color: var(--m-color-white);
      cursor: pointer;

      &:hover {
        background-color: black;
        color: var(--m-color-white);
      }
    }
  }
</style>

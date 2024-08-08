import { invoke } from "@tauri-apps/api/tauri";
import { writable } from "svelte/store";
import type { AnimeDetail } from "./bindings/AnimeDetail";
import type { SearchResults } from "./types";

export const currentPage = writable<number>(1);
export const searchResults = writable<SearchResults>({
  query: "",
  data: [],
  lastVisiblePage: 0,
});
export const topAnime = writable<AnimeDetail[]>([]);

function createWatchWatchedList() {
  const { subscribe, set, update } = writable<AnimeDetail[]>([]);
  (async () => set(await invoke("get_watch_and_watched")))();
  return {
    subscribe,
    update,
    updateList: async () => set(await invoke("get_watch_and_watched")),
    clear: () => set([]),
  };
}

function createShowAnimeDetail() {
  const { subscribe, set } = writable(false);
  return {
    subscribe,
    show: () => set(true),
    hide: () => set(false),
  };
}

export const watchWatchedList = createWatchWatchedList();
export const showAnimeDetail = createShowAnimeDetail();

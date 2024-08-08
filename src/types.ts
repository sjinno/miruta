import type { AnimeDetail } from "./bindings/AnimeDetail";

export enum ListType {
  Watchlist = "watchlist",
  Watched = "watched",
  TopAnime = "top anime",
  Recommendations = "recommendations",
}

export interface SearchResults {
  query: string;
  data: AnimeDetail[];
  lastVisiblePage: number;
}

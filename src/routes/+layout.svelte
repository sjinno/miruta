<script lang="ts">
  import { onMount } from "svelte";

  import {
    getCurrent,
    LogicalPosition,
    LogicalSize,
  } from "@tauri-apps/api/window";
  import { invoke } from "@tauri-apps/api/tauri";

  import "../app.scss";
  import { showAnimeDetail } from "../store";

  import Header from "../components/Header.svelte";
  import Footer from "../components/Footer.svelte";

  import type { UnlistenFn } from "@tauri-apps/api/event";
  import type { WindowState } from "../bindings/WindowState";

  const appWindow = getCurrent();

  async function getSavedWindowState() {
    try {
      const state = (await invoke("get_window_state")) as WindowState;
      appWindow.setSize(new LogicalSize(state.w, state.h));
      appWindow.setPosition(new LogicalPosition(state.x, state.y));
    } catch (error) {
      console.error("Error getting window state:", error);
    }
  }

  async function saveWindowState() {
    const sf = await appWindow.scaleFactor();
    const { width: w, height: h } = (await appWindow.outerSize()).toLogical(sf);
    const { x, y } = (await appWindow.outerPosition()).toLogical(sf);
    const state = { w, h, x, y };
    try {
      await invoke("save_window_state", { state });
    } catch (error) {
      console.error("Error saving window windowState:", error);
    }
  }

  function handleCloseModal(e: KeyboardEvent) {
    if (e.key === "Escape") showAnimeDetail.hide();
  }

  onMount(() => {
    let unlistenResize: UnlistenFn;
    let unlistenMove: UnlistenFn;

    (async () => {
      await getSavedWindowState();
      unlistenResize = await appWindow.onResized(saveWindowState);
      unlistenMove = await appWindow.onMoved(saveWindowState);
    })();

    window.addEventListener("keydown", handleCloseModal);

    return () => {
      unlistenResize();
      unlistenMove();
      window.removeEventListener("keydown", handleCloseModal);
    };
  });
</script>

<div class="flex">
  <div class="flex-1">
    <Header />
  </div>
  <div class="flex-2">
    <slot></slot>
  </div>
  <div class="flex-3">
    <Footer />
  </div>
</div>

<style lang="scss">
  .flex {
    width: 100%;
    height: 100%;

    display: flex;
    flex-direction: column;

    &-2 {
      flex-grow: 1;
    }

    &-3 {
      height: 6rem;
    }
  }
</style>

<script lang="ts">
  import { open } from "@tauri-apps/api/shell";

  async function openLink(url: string) {
    try {
      await open(url);
      console.log("Link opened successfully");
    } catch (error) {
      console.error("Failed to open link:", error);
    }
  }
</script>

<footer>
  Powered by&nbsp;<span
    class="open-link-span"
    on:click={() => openLink("https://jikan.moe")}
    aria-hidden="true">Jikan API</span
  >
</footer>

<style lang="scss">
  footer {
    background-color: var(--m-color-gray);
    text-align: center;
    text-transform: none;
    width: 100%;
    height: 6rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: var(--m-fs-sans-s);

    .open-link-span {
      font-weight: var(--m-fw-sans-xb);

      &:hover {
        cursor: pointer;
        text-decoration: underline;
        color: var(--m-color-blue);
      }
    }
  }
</style>

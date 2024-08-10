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
    class="attribute"
    on:click={() => openLink("https://jikan.moe")}
    aria-hidden="true">Jikan API</span
  >. Icons by&nbsp;
  <span
    class="attribute"
    on:click={() => openLink("https://icons8.com")}
    aria-hidden="true">Icons8</span
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

    .attribute {
      font-weight: var(--m-fw-sans-xb);

      &:hover {
        cursor: pointer;
        text-decoration: underline;
        color: var(--m-color-blue);
      }
    }
  }
</style>

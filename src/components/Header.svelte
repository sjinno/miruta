<script>
  import Nav from "./Nav.svelte";
</script>

<header>
  <h1><a href="/" class="title">Miruta</a></h1>
  <div class="nav">
    <Nav />
  </div>
</header>

<style lang="scss">
  .title {
    font-family: var(--m-font-sans);
    font-size: var(--m-fs-sans-l);
    font-weight: var(--m-fw-sans-xxxb);
  }

  header {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2rem;
  }
</style>

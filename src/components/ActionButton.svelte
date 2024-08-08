<script lang="ts">
  export let className: string;
  export let label: string;
  export let handleClick: () => void;
</script>

<button on:click|stopPropagation={handleClick} class="{className} button-12">
  <span>{label}</span></button
>

<style lang="scss">
  button {
    width: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    font-size: var(--m-fs-sans-xs);
    font-weight: var(--m-fw-sans-b);
  }

  .watchlist {
    &--no {
      background-color: var(--m-color-yellow);
      color: black;
    }

    &--yes {
      background-color: var(--m-color-red);
      color: var(--m-color-white);
      position: relative;

      &::before {
        content: "Unwatchlist?";
        opacity: 0;
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        font-weight: var(--m-fw-sans-xxl);
      }

      &:hover {
        background-color: black;
        transition: background-color 0.3s ease;

        &::before {
          opacity: 1;
          transition: opacity 0.3s ease;
        }
      }

      & span {
        opacity: 1;
      }

      &:hover span {
        opacity: 0;
        transition: opacity 0.3s ease;
      }
    }
  }

  .watched {
    &--no {
      background-color: var(--m-color-green);
      color: black;
    }

    &--yes {
      background-color: var(--m-color-purple);
      color: var(--m-color-white);
      position: relative;

      &::before {
        content: "Unwatch?";
        opacity: 0;
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        font-weight: var(--m-fw-sans-xxl);
      }

      &:hover {
        background-color: var(--m-color-black);
        transition: background-color 0.3s ease;

        &::before {
          opacity: 1;
          transition: opacity 0.3s ease;
        }
      }

      & span {
        opacity: 1;
      }

      &:hover span {
        opacity: 0;
        transition: opacity 0.3s ease;
      }
    }
  }

  .delete {
    background-color: #9a8c98;
    color: var(--m-color-white);
  }
</style>

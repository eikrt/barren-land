<script>
  import { onMount } from 'svelte';
  import { createScene } from "./scene";

  let el;

  onMount(() => {
    createScene(el)
  });
</script>
<svelte:head>
    <title> Barren Lands </title>

</svelte:head>

<canvas bind:this={el}></canvas>

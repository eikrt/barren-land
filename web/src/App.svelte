<script>
  import { onMount } from 'svelte';
  import { createScene } from "./scene";
  import { start } from "./start";

  let el;
  let p;
  onMount(() => {
   createScene(el)
  });
</script>
<svelte:head>
    <title> Barren Lands </title>
</svelte:head>

<h1> Barren Lands </h1>
<div id = loginScreen>
<h2> Login </h2>
  <label for="fname">Name:</label>
  <input type="text" id="fname" name="fname"><br><br>
  <label for="lname">Password:</label>
  <input type="text" id="lname" name="lname"><br><br>
  <button id="playButton"> Play! </button>
</div>
<canvas id=mainCanvas bind:this={el}></canvas>

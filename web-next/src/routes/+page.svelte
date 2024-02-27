<script lang="ts">
  import { PUBLIC_UNDER_CONSTRUCTION } from "$env/static/public";

  import type { PageData } from "./$types";

  export let data: PageData;

  import { Input } from "$lib/components/ui/input";
  import { Button } from "$lib/components/ui/button";
  import { Sun, Moon } from "radix-icons-svelte";
  import { toggleMode } from "mode-watcher";

  import Chart from "$lib/components/chart.svelte";
</script>

<div class="m-5">
  <div class="flex flex-row justify-between">
    <div class="basis-1/4 font-kanit font-medium text-2xl">PoE Ledger</div>
    <div class="basis-1/2">
      <form method="get" class="flex flex-row justify-center gap-5">
        <Input
          name="item"
          class="max-w-lg font-kanit"
          placeholder="Search uniques..."
        />
        <Button class="font-kanit" type="submit">Search</Button>
      </form>
    </div>
    <div class="basis-1/4">
      <div class="flex justify-end">
        <Button on:click={toggleMode} variant="outline" size="icon">
          <Sun
            class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
          />
          <Moon
            class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
          />
          <span class="sr-only">Toggle theme</span>
        </Button>
      </div>
    </div>
  </div>

  <div class="flex justify-center">
    <div class="relative mt-20" style="width: 80vw; height: 50vh;">
      {#if PUBLIC_UNDER_CONSTRUCTION === "true"}
        <h1>UNDER CONSTRUCTION</h1>
      {:else}
        <Chart {data} />
      {/if}
    </div>
  </div>
</div>

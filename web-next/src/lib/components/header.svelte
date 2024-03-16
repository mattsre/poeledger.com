<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { Sun, Moon } from "svelte-radix";
  import { mode, setMode } from "mode-watcher";

  const toggleTheme = (e: MouseEvent) => {
    e.preventDefault();

    if ($mode === "dark") {
      setMode("light");
    } else {
      setMode("dark");
    }
  };
</script>

<div class="flex flex-row justify-between">
  <div class="basis-1/4 font-kanit font-medium text-2xl">PoE Ledger</div>
  <div class="basis-1/4">
    <div class="flex justify-end">
      <Button on:click={toggleTheme} variant="outline" size="icon">
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

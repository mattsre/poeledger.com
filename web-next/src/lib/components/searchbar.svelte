<script lang="ts" context="module">
  export interface SearchItem {
    id: string;
    name: string;
  }
</script>

<script lang="ts">
  export let form: SuperForm<any>;
  export let searchIndex: string;

  import { MeiliSearch } from "meilisearch";
  import {
    PUBLIC_MEILISEARCH_URL,
    PUBLIC_MEILISEARCH_API_KEY,
  } from "$env/static/public";

  import * as Form from "$lib/components/ui/form";
  import * as Popover from "$lib/components/ui/popover";
  import * as Command from "$lib/components/ui/command";

  import { buttonVariants } from "$lib/components/ui/button";
  import { tick } from "svelte";
  import { cn } from "$lib/utils";
  import type { SuperForm } from "sveltekit-superforms";
  import { Input } from "$lib/components/ui/input";

  const { form: formData } = form;

  const meiliClient = new MeiliSearch({
    host: PUBLIC_MEILISEARCH_URL,
    apiKey: PUBLIC_MEILISEARCH_API_KEY,
  });

  let itemSearchOpen = false;
  const closeAndFocusTrigger = (triggerId: string) => {
    itemSearchOpen = false;
    tick().then(() => {
      document.getElementById(triggerId)?.focus();
    });
  };

  const debounce = (callback: Function, wait = 300) => {
    let timeout: ReturnType<typeof setTimeout>;

    return (...args: any[]) => {
      clearTimeout(timeout);
      timeout = setTimeout(() => callback(...args), wait);
    };
  };

  let foundItems: SearchItem[] = [];
  const searchItems = async (event: KeyboardEvent) => {
    const input = event?.target as HTMLInputElement;
    const query = input.value;

    if (!query || query.length === 0) {
      foundItems = [];
      return;
    }

    if (searchIndex) {
      const meiliIndex = meiliClient.index(searchIndex);
      const results = await meiliIndex.search(query, {
        limit: 10,
      });

      foundItems = results.hits as SearchItem[];
    }
  };
</script>

<Form.Field {form} name="item" class="flex flex-col">
  <Popover.Root bind:open={itemSearchOpen} let:ids>
    <Form.Control let:attrs>
      <Form.Label class="font-kanit">Item Name</Form.Label>
      <Popover.Trigger
        class={cn(
          buttonVariants({ variant: "outline" }),
          "max-w-[300px] justify-between",
          !$formData.item && "text-muted-foreground",
        )}
        role="combobox"
        {...attrs}
      >
        {$formData.item}
      </Popover.Trigger>
      <input hidden value={$formData.item} name={attrs.name} />
    </Form.Control>
    <Popover.Content class="max-w-[300px] p-0">
      <Command.Root>
        <Input
          autofocus
          placeholder="Search items..."
          class="h-9 font-kanit focus-visible:outline-none focus-visible:ring-0 focus-visible:ring-transparent"
          type="search"
          on:keyup={debounce(searchItems)}
        />
        <Command.Empty class="font-kanit">No items found.</Command.Empty>
        <Command.Group>
          {#if foundItems.length > 0}
            {#each foundItems as item}
              <Command.Item
                value={item.id}
                onSelect={() => {
                  $formData.item = item.name;
                  closeAndFocusTrigger(ids.trigger);
                }}
                class="font-kanit"
              >
                {item.name}
              </Command.Item>
            {/each}
          {/if}
        </Command.Group>
      </Command.Root>
    </Popover.Content>
  </Popover.Root>
  <Form.FieldErrors />
</Form.Field>

<script lang="ts" context="module">
  import { z } from "zod";

  export const itemSearchFormSchema = z.object({
    item: z.string().default("Headhunter"),
    league: z.string().default("Affliction"),
    intervalAmount: z.coerce.number().gt(0, "Interval must be greater than 0"),
    intervalUnit: z.string().default("hour"),
    tenthQuantile: z.boolean().default(true),
    fifteenthQuantile: z.boolean().default(false).optional(),
    thirtiethQuantile: z.boolean().default(false).optional(),
    customQuantiles: z
      .array(
        z.coerce
          .number()
          .gt(0, "Quantile must be greater than 0%")
          .lt(100, "Quantile must be less than 100%"),
      )
      .default([]),
    startTime: z.string(),
    endTime: z.string(),
  });

  export type ItemSearchFormData = z.infer<typeof itemSearchFormSchema>;
  export type ItemSearchFormSchema = typeof itemSearchFormSchema;
</script>

<script lang="ts">
  import type { Selected } from "bits-ui";
  import type { SuperValidated, Infer } from "sveltekit-superforms";

  import { browser, dev } from "$app/environment";

  import * as Form from "$lib/components/ui/form";
  import * as Select from "$lib/components/ui/select";
  import * as Collapsible from "$lib/components/ui/collapsible";

  import SuperDebug, { superForm } from "sveltekit-superforms";
  import { Input } from "$lib/components/ui/input";
  import { Switch } from "$lib/components/ui/switch";
  import { zodClient } from "sveltekit-superforms/adapters";
  import { Button } from "$lib/components/ui/button";
  import { ChevronDown, Cross2 } from "svelte-radix";
  import { tick } from "svelte";
  import { cn } from "$lib/utils";
  import Searchbar from "$lib/components/searchbar.svelte";

  export let data: SuperValidated<Infer<ItemSearchFormSchema>>;
  export let leagues: any;

  const form = superForm(data, {
    validators: zodClient(itemSearchFormSchema),
  });

  export let { form: formData, enhance } = form;

  const toSelected = (value: string): Selected<string> => {
    const label = value.charAt(0).toUpperCase() + value.slice(1);

    return {
      label,
      value,
    };
  };

  const addCustomPriceQuantile = () => {
    $formData.customQuantiles = [...$formData.customQuantiles, 10];

    tick().then(() => {
      const quantileInputs = Array.from(
        document.querySelectorAll<HTMLElement>(
          "#item-search input[name='customQuantiles']",
        ),
      );

      const lastInput = quantileInputs[quantileInputs.length - 1];
      lastInput && lastInput.focus();
    });
  };

  const removeCustomPriceQuantile = (i: number) => {
    $formData.customQuantiles = $formData.customQuantiles.filter(
      (_, idx) => i !== idx,
    );
  };
</script>

<form id="item-search" class="flex flex-col gap-5 max-w-md">
  <h1 class="font-kanit font-medium text-lg">Item Search</h1>

  <Searchbar {form} searchIndex="uniques" />

  <Form.Field {form} name="league" class="font-kanit">
    <Form.Control let:attrs>
      <Form.Label>League</Form.Label>
      <Select.Root
        selected={toSelected($formData.league)}
        onSelectedChange={(v) => {
          v && ($formData.league = v.value);
        }}
      >
        <Select.Trigger {...attrs} class="w-[180px]">
          <Select.Value placeholder="Select a league" />
        </Select.Trigger>
        <Select.Content class="font-kanit">
          {#each leagues as league}
            <Select.Item value={league.name} label={league.name}
              >{league.name}</Select.Item
            >
          {/each}
        </Select.Content>
      </Select.Root>
      <input hidden bind:value={$formData.intervalUnit} name={attrs.name} />
    </Form.Control>
    <Form.FieldErrors />
  </Form.Field>

  <div class="flex flex-col gap-4">
    <p class="font-kanit font-medium text-sm">Price Quantiles</p>
    <Form.Field
      {form}
      name="tenthQuantile"
      class="flex flex-row items-center justify-between rounded-lg border p-4 font-kanit"
    >
      <Form.Control let:attrs>
        <div class="space-y-0.5">
          <Form.Label>10th Quantile</Form.Label>
          <Form.Description
            >Fetch the price 10% from the bottom</Form.Description
          >
        </div>
        <Switch
          includeInput
          {...attrs}
          bind:checked={$formData.tenthQuantile}
        />
      </Form.Control>
    </Form.Field>

    <Form.Field
      {form}
      name="fifteenthQuantile"
      class="flex flex-row items-center justify-between rounded-lg border p-4 font-kanit"
    >
      <Form.Control let:attrs>
        <div class="space-y-0.5">
          <Form.Label>15th Quantile</Form.Label>
          <Form.Description
            >Fetch the price 15% from the bottom</Form.Description
          >
        </div>
        <Switch
          includeInput
          {...attrs}
          bind:checked={$formData.fifteenthQuantile}
        />
      </Form.Control>
    </Form.Field>

    <Form.Field
      {form}
      name="thirtiethQuantile"
      class="flex flex-row items-center justify-between rounded-lg border p-4 font-kanit"
    >
      <Form.Control let:attrs>
        <div class="space-y-0.5">
          <Form.Label>30th Quantile</Form.Label>
          <Form.Description
            >Fetch the price 30% from the bottom</Form.Description
          >
        </div>
        <Switch
          includeInput
          {...attrs}
          bind:checked={$formData.thirtiethQuantile}
        />
      </Form.Control>
    </Form.Field>
  </div>

  <Form.Button>Search</Form.Button>

  <Collapsible.Root class="space-y-2">
    <Collapsible.Trigger asChild let:builder>
      <Button
        builders={[builder]}
        variant="ghost"
        size="sm"
        class="font-kanit font-medium text-md"
      >
        Advanced Settings &nbsp; <ChevronDown />
        <span class="sr-only">Toggle</span>
      </Button>
    </Collapsible.Trigger>
    <Collapsible.Content>
      <div class="flex flex-col gap-4">
        <div class="flex flex-row gap-4">
          <Form.Field {form} name="intervalAmount" class="font-kanit">
            <Form.Control let:attrs>
              <Form.Label>Interval Amount</Form.Label>
              <Input
                {...attrs}
                bind:value={$formData.intervalAmount}
                type="number"
              />
            </Form.Control>
            <Form.FieldErrors />
          </Form.Field>

          <Form.Field {form} name="intervalUnit" class="font-kanit">
            <Form.Control let:attrs>
              <Form.Label>Interval Unit</Form.Label>
              <Select.Root
                selected={toSelected($formData.intervalUnit)}
                onSelectedChange={(v) => {
                  v && ($formData.intervalUnit = v.value);
                }}
              >
                <Select.Trigger {...attrs} class="w-[180px]">
                  <Select.Value placeholder="Select a unit" />
                </Select.Trigger>
                <Select.Content class="font-kanit">
                  <Select.Item value="minute" label="Minute">Minute</Select.Item
                  >
                  <Select.Item value="hour" label="Hour">Hour</Select.Item>
                  <Select.Item value="day" label="Day">Day</Select.Item>
                  <Select.Item value="week" label="Week">Week</Select.Item>
                  <Select.Item value="month" label="Month">Month</Select.Item>
                  <Select.Item value="year" label="Year">Year</Select.Item>
                </Select.Content>
              </Select.Root>
              <input
                hidden
                bind:value={$formData.intervalUnit}
                name={attrs.name}
              />
            </Form.Control>
            <Form.FieldErrors />
          </Form.Field>
        </div>

        <div>
          <Form.Fieldset {form} name="customQuantiles" class="font-kanit">
            <Form.Legend>Custom Price Quantiles</Form.Legend>
            {#each $formData.customQuantiles as _, i}
              <Form.ElementField {form} name="customQuantiles[{i}]">
                <Form.Description class={cn(i !== 0 && "sr-only")}>
                  Set custom price quantiles to calculate
                </Form.Description>
                <Form.Control let:attrs>
                  <div class="flex flex-row gap-2 max-w-xs">
                    <Input
                      {...attrs}
                      bind:value={$formData.customQuantiles[i]}
                    />
                    <Button
                      size="icon"
                      variant="outline"
                      on:click={() => removeCustomPriceQuantile(i)}
                    >
                      <Cross2 />
                    </Button>
                  </div>
                </Form.Control>
                <Form.FieldErrors />
              </Form.ElementField>
            {/each}
          </Form.Fieldset>
          <Button
            type="button"
            size="sm"
            class="mt-2"
            on:click={addCustomPriceQuantile}
          >
            Add Quantile
          </Button>
        </div>

        <Form.Field {form} name="startTime" class="font-kanit">
          <Form.Control let:attrs>
            <Form.Label>Start Time</Form.Label>
            <Input {...attrs} bind:value={$formData.startTime} />
          </Form.Control>
          <Form.Description
            >A unix timestamp for when to start fetching data from</Form.Description
          >
          <Form.FieldErrors />
        </Form.Field>

        <Form.Field {form} name="endTime" class="font-kanit">
          <Form.Control let:attrs>
            <Form.Label>End Time</Form.Label>
            <Input {...attrs} bind:value={$formData.endTime} />
          </Form.Control>
          <Form.FieldErrors />
          <Form.Description
            >A unix timestamp for when to stop fetching data</Form.Description
          >
        </Form.Field>
      </div>
    </Collapsible.Content>
  </Collapsible.Root>

  {#if browser && dev}
    <SuperDebug data={$formData} />
  {/if}
</form>

<script lang="ts">
  import { browser, dev } from "$app/environment";
  import * as Form from "$lib/components/ui/form";
  import * as Select from "$lib/components/ui/select";
  import * as Collapsible from "$lib/components/ui/collapsible";
  import { Switch } from "$lib/components/ui/switch";
  import { Input } from "$lib/components/ui/input";
  import { itemSearchFormSchema, type ItemSearchFormSchema } from "./schema";
  import SuperDebug, {
    type SuperValidated,
    type Infer,
    superForm,
  } from "sveltekit-superforms";
  import { zodClient } from "sveltekit-superforms/adapters";
  import { Button } from "$lib/components/ui/button";
  import { ChevronDown } from "radix-icons-svelte";

  export let data: SuperValidated<Infer<ItemSearchFormSchema>>;

  const form = superForm(data, {
    validators: zodClient(itemSearchFormSchema),
  });

  export let { form: formData, enhance } = form;

  $: selectedIntervalUnit = $formData.intervalUnit
    ? {
        label:
          $formData.intervalUnit.charAt(0).toUpperCase() +
          $formData.intervalUnit.slice(1),
        value: $formData.intervalUnit,
      }
    : undefined;
</script>

<form class="flex flex-col justify-center gap-5 max-w-md">
  <h1 class="font-kanit font-medium text-lg">Item Search</h1>
  <Form.Field {form} name="item" class="font-kanit">
    <Form.Control let:attrs>
      <Form.Label>Item Name</Form.Label>
      <Input {...attrs} bind:value={$formData.item} />
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
        Advanced Filtering &nbsp; <ChevronDown />
        <span class="sr-only">Toggle</span>
      </Button>
    </Collapsible.Trigger>
    <Collapsible.Content>
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
              selected={selectedIntervalUnit}
              onSelectedChange={(v) => {
                v && ($formData.intervalUnit = v.value);
              }}
            >
              <Select.Trigger {...attrs} class="w-[180px]">
                <Select.Value placeholder="Select a unit" />
              </Select.Trigger>
              <Select.Content class="font-kanit">
                <Select.Item value="minute" label="Minute">Minute</Select.Item>
                <Select.Item value="hour" label="Hour">Hour</Select.Item>
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
    </Collapsible.Content>
  </Collapsible.Root>

  <!-- {#if browser && dev}
    <SuperDebug data={$formData} />
  {/if} -->
</form>

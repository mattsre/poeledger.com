<script lang="ts">
  import type { PageData } from "./$types";
  export let data: PageData;

  import { PUBLIC_API_HOST } from "$env/static/public";
  import Header from "$lib/components/header.svelte";
  import Chart from "$lib/components/chart.svelte";
  import ItemSearch, {
    type ItemSearchFormData,
  } from "$lib/components/item-search.svelte";

  let chartDataset = data.initialPriceHistory;
  let formData: any = undefined;

  const handleItemSearch = async () => {
    if ($formData.item) {
      const fd: ItemSearchFormData = $formData;

      let historyUrl = `${PUBLIC_API_HOST}/history?item=${fd.item.trim()}&league=${fd.league}`;
      if (fd.intervalAmount && fd.intervalUnit) {
        historyUrl = `${historyUrl}&intervalAmount=${fd.intervalAmount}&intervalUnit=${fd.intervalUnit}`;
      }

      if (fd.startTime && fd.endTime) {
        historyUrl = `${historyUrl}&startTime=${fd.startTime}&endTime=${fd.endTime}`;
      }

      if (fd.tenthQuantile) {
        historyUrl = `${historyUrl}&quantiles=0.1`;
      }

      if (fd.fifteenthQuantile) {
        historyUrl = `${historyUrl}&quantiles=0.15`;
      }

      if (fd.thirtiethQuantile) {
        historyUrl = `${historyUrl}&quantiles=0.3`;
      }

      if (fd.customQuantiles.length > 0) {
        for (let q of fd.customQuantiles) {
          let floatQ = (q as number) / 100;

          historyUrl = `${historyUrl}&quantiles=${floatQ}`;
        }
      }

      const response = await fetch(historyUrl);
      if (response.ok) {
        chartDataset = await response.json();
      } else {
        console.error(`failed to request to ${historyUrl}`);
        console.error(response);
      }
    }
  };
</script>

<svelte:head>
  <title>PoE Ledger | Item Price History</title>
</svelte:head>

<div class="p-5">
  <Header />

  <div class="mt-12 grid gap-4 grid-cols-4">
    <div class="col-span-1" on:submit|preventDefault={handleItemSearch}>
      <ItemSearch data={data.form} bind:formData leagues={data.leagues} />
    </div>
    <div class="col-span-3">
      <Chart data={chartDataset} />
    </div>
  </div>
</div>

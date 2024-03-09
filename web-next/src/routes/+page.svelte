<script lang="ts">
  import type { PageData } from "./$types";
  export let data: PageData;

  import Header from "$lib/components/header.svelte";
  import ItemSearch from "./item-search.svelte";
  import Chart from "$lib/components/chart.svelte";

  let chartDataset = data.initialPriceHistory;
  let formData: any = undefined;

  const handleItemSearch = async () => {
    if ($formData.item) {
      const {
        item,
        intervalAmount,
        intervalUnit,
        tenthQuantile,
        fifteenthQuantile,
        thirtiethQuantile,
        startTime,
        endTime,
      } = $formData;

      let historyUrl = `http://localhost:3000/history?item=${item.trim()}`;
      if (intervalAmount && intervalUnit) {
        historyUrl = `${historyUrl}&intervalAmount=${intervalAmount}&intervalUnit=${intervalUnit}`;
      }

      if (startTime && endTime) {
        historyUrl = `${historyUrl}&startTime=${startTime}&endTime=${endTime}`;
      }

      if (tenthQuantile) {
        historyUrl = `${historyUrl}&quantiles=0.1`;
      }

      if (fifteenthQuantile) {
        historyUrl = `${historyUrl}&quantiles=0.15`;
      }

      if (thirtiethQuantile) {
        historyUrl = `${historyUrl}&quantiles=0.3`;
      }

      const response = await fetch(historyUrl);
      if (response.ok) {
        const jdata = await response.json();
        console.log(jdata);
        chartDataset = jdata;
      } else {
        console.error(`failed to request to ${historyUrl}`);
        console.error(response);
      }
    }
  };
</script>

<div class="p-5">
  <Header />

  <div class="mt-12 grid gap-4 grid-cols-4">
    <div class="col-span-1" on:submit|preventDefault={handleItemSearch}>
      <ItemSearch data={data.form} bind:formData />
    </div>
    <div class="col-span-3">
      <Chart data={chartDataset} />
    </div>
  </div>
</div>

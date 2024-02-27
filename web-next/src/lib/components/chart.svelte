<script lang="ts">
  export let data: any;

  import { onMount } from "svelte";
  import {
    Chart,
    LineController,
    LineElement,
    PointElement,
    CategoryScale,
    LinearScale,
  } from "chart.js";

  Chart.register(
    LineController,
    LineElement,
    PointElement,
    CategoryScale,
    LinearScale,
  );

  let chartCanvas: HTMLCanvasElement;

  const renderPriceChart = () => {
    console.log("rendering price chart");
    let ctx = chartCanvas.getContext("2d");

    if (ctx && data) {
      let labelData = [];
      let priceData = [];

      for (let event of data.events) {
        labelData.push(event.listedDate);
        priceData.push(event.listedPrice);
      }

      new Chart(ctx, {
        type: "line",
        data: {
          labels: labelData,
          datasets: [
            {
              label: data.itemName,
              data: priceData,
            },
          ],
        },
        options: {
          maintainAspectRatio: false,
          backgroundColor: "rgb(75, 192, 192)",
          borderColor: "rgb(75, 192, 192)",
        },
      });
    }
  };

  onMount(() => renderPriceChart());
</script>

<canvas bind:this={chartCanvas}></canvas>

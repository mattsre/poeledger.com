<script lang="ts">
  export let data: any;

  import { mode } from "mode-watcher";
  import { onMount, afterUpdate } from "svelte";
  import Chart from "chart.js/auto";

  interface PriceQuantiles {
    [key: string]: [number];
  }

  let chartCanvas: HTMLCanvasElement;
  let chartInstance: Chart;

  mode.subscribe((m) => {
    if (m === "light") {
      Chart.defaults.color = "black";
      for (let id in Chart.instances) {
        let ch = Chart.instances[id];

        ch.options.scales!.y!.grid!.color = "rgba(0, 0, 0, 0.3)";
        ch.options.scales!.y!.ticks!.color = "black";
        ch.options.scales!.x!.grid!.color = "rgba(0, 0, 0, 0.3)";
        ch.options.scales!.x!.ticks!.color = "black";

        ch.update();
      }
    } else {
      Chart.defaults.color = "white";
      for (let id in Chart.instances) {
        let ch = Chart.instances[id];

        ch.options.scales!.y!.grid!.color = "rgba(255, 255, 255, 0.3)";
        ch.options.scales!.y!.ticks!.color = "white";
        ch.options.scales!.x!.grid!.color = "rgba(255, 255, 255, 0.3)";
        ch.options.scales!.x!.ticks!.color = "white";

        ch.update();
      }
    }
  });

  const createPriceChart = () => {
    let ctx = chartCanvas.getContext("2d");

    if (ctx && data) {
      Chart.defaults.font.family = "Kanit";
      Chart.defaults.font.size = 14;

      chartInstance = new Chart(ctx, {
        type: "line",
        data: {
          labels: [],
          datasets: [],
        },
        options: {
          maintainAspectRatio: true,
          scales: {
            y: {
              type: "linear",
              beginAtZero: true,
              grace: "20%",
              ticks: {
                callback(tickValue, index, ticks) {
                  return tickValue + "d";
                },
              },
            },
          },
        },
      });

      if (data.length && data.length > 0) {
        let { labels, prices, subtitleText } = formatDataForChart(data);

        chartInstance.data.labels = labels;
        chartInstance.data.datasets = prices;
        chartInstance.options.plugins!.subtitle = {
          display: true,
          text: subtitleText,
          font: {
            size: 20,
          },
        };
      }
    }
  };

  const updateChartData = () => {
    if (data.length && data.length > 0) {
      let { labels, prices, subtitleText } = formatDataForChart(data);

      chartInstance.data.labels = labels;
      chartInstance.data.datasets = prices;
      chartInstance.options.plugins!.subtitle!.text = subtitleText;

      chartInstance.update();
    } else {
      console.warn("recieved empty price history: ", data);
    }
  };

  const formatDataForChart = (chartData: any) => {
    let labels = [];
    let priceQuantiles: PriceQuantiles = {};

    for (let point of chartData) {
      if (point.listed_currency === "divine") {
        labels.push(new Date(point.interval_bucket * 1000).toLocaleString());

        for (let [quantile, price] of point.price_by_quantile) {
          if (priceQuantiles[`${quantile}`]) {
            priceQuantiles[`${quantile}`].push(price);
          } else {
            priceQuantiles[`${quantile}`] = [price];
          }
        }
      }
    }

    const prices = Object.entries(priceQuantiles).map(
      ([quantile, prices]: any) => {
        return {
          label: `${quantile * 100}th Quantile`,
          data: prices,
          tension: 0.1,
        };
      },
    );

    const subtitleText = `${chartData[0].item_name} Price History`;

    return {
      labels,
      prices,
      subtitleText,
    };
  };

  onMount(() => {
    createPriceChart();
    updateChartData();
  });

  afterUpdate(() => updateChartData());
</script>

<canvas bind:this={chartCanvas}></canvas>

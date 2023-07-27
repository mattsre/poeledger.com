import { json, type V2_MetaFunction } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
} from 'chart.js';
import { Line } from "react-chartjs-2";


const recordsToChartData = (records: any): any => {
  let labels: any = [];
  let dataset: any = [];

  for (let record of records) {
    labels.push(record.date);
    dataset.push(record.value);
  }

  return {
    labels,
    datasets: [
      {
        label: "Divine Orbs",
        data: dataset,
        borderColor: 'rgb(255, 99, 132)',
        backgroundColor: 'rgba(255, 99, 132, 0.5)',
      }
    ]
  }
}

export const meta: V2_MetaFunction = () => {
  return [
    { title: "PoE Ledger" },
    { name: "description", content: "A historical price tracker for Path of Exile" },
  ];
};

export async function loader() {
  const request = await fetch("http://127.0.0.1:5000");
  const records = await request.json();

  return json(records);
}

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend
);

export const options = {
  responsive: true,
  plugins: {
    legend: {
      position: 'top' as const,
    },
    title: {
      display: true,
      text: 'PoE Price Chart',
    },
  },
};


export default function Index() {
  const priceRecords = useLoaderData<typeof loader>();
  console.log(priceRecords);

  const chartData = recordsToChartData(priceRecords);

  return (
    <div>
      <h1>Welcome to poeledger.com!</h1>
      <Line options={options} data={chartData} />
    </div>
  );
}

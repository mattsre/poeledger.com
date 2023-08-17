import { AppShell, Header } from "@mantine/core";
import { PriceRecord } from "@poeledger/economy-data";
import { LoaderArgs, json, type V2_MetaFunction } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";
import {
	CategoryScale,
	Chart as ChartJS,
	Legend,
	LineElement,
	LinearScale,
	PointElement,
	Title,
	Tooltip,
} from "chart.js";
import { Line } from "react-chartjs-2";
import HeaderWithSearch from "src/components/HeaderWithSearch";

const recordsToChartData = (data: LoaderResponse): any => {
	let labels = [];
	let dataset = [];

	for (let record of data.records) {
		labels.push(record.date.toString());
		dataset.push(record.value);
	}

	return {
		labels,
		datasets: [
			{
				label: data.currentItem,
				data: dataset,
				borderColor: "rgb(255, 99, 132)",
				backgroundColor: "rgba(255, 99, 132, 0.5)",
			},
		],
	};
};

export const meta: V2_MetaFunction = () => {
	return [
		{ title: "PoE Ledger" },
		{
			name: "description",
			content: "A historical price tracker for Path of Exile",
		},
	];
};

interface LoaderResponse {
	currentItem: string
	currentLeague: string
	records: PriceRecord[]
	filters: string[]
	leagues: string[]
}

export const loader = async ({ request }: LoaderArgs) => {
	let backendHost = process.env.BACKEND_HOST;

	if (backendHost) {
		const url = new URL(request.url);
		const currentItem = url.searchParams.get("name") || "Divine Orb";
		const currentLeague = url.searchParams.get("league") || "Sanctum";

		const dataApiUri = `${backendHost}/prices?name=${currentItem}&league=${currentLeague}`;
		const dataApiRequest = await fetch(dataApiUri);
		const records: PriceRecord[] = await dataApiRequest.json();

		const filtersRequest = await fetch(
			`${backendHost}/filters`,
		);
		const filters: string[] = await filtersRequest.json();

		const leaguesRequest = await fetch(
			`${backendHost}/leagues`,
		);
		const leagues: string[] = await leaguesRequest.json();

		return json({
			currentItem,
			currentLeague,
			records,
			filters,
			leagues
		});
	}


	return json([]);
};

ChartJS.register(
	CategoryScale,
	LinearScale,
	PointElement,
	LineElement,
	Title,
	Tooltip,
	Legend,
);

export default function Index() {
	const data = useLoaderData<LoaderResponse>();
	const chartDatasets = recordsToChartData(data);

	return (
		<AppShell
			padding="md"
			header={
				<Header height={120} p="md">
					{<HeaderWithSearch />}
				</Header>
			}
		>
			<Line
				options={{
					responsive: true,
					interaction: {
						intersect: false,
					},
					plugins: {
						legend: {
							position: "top",
						},
						title: {
							display: true,
							text: `${data.currentLeague} - ${data.currentItem} Prices`,
						},
					},
				}}
				data={chartDatasets}
			/>
		</AppShell>
	);
}

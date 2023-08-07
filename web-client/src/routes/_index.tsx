import { AppShell, Header, Navbar } from "@mantine/core";
import { json, LoaderArgs, type V2_MetaFunction } from "@remix-run/node";
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
} from "chart.js";
import { Line } from "react-chartjs-2";
import HeaderWithSearch from "src/components/HeaderWithSearch";
import SidebarWithFilters from "src/components/SidebarWithFilters";

const recordsToChartData = (data: any): any => {
	let labels: any = [];
	let dataset: any = [];

	for (let record of data.records) {
		labels.push(record.date);
		dataset.push(record.value);
	}

	return {
		labels,
		datasets: [
			{
				label: data.item,
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

export const loader = async ({ request }: LoaderArgs) => {
	let backendHost = process.env.BACKEND_HOST;

	if (backendHost) {
		const url = new URL(request.url);
		const getItem = url.searchParams.get("get") || "Divine Orb";
		const desiredLeague = url.searchParams.get("league") || "Sanctum";

		const dataApiRequest = await fetch(
			`${backendHost}/prices?get=${getItem}&pay=Chaos%20Orb&league=${desiredLeague}`,
		);
		const records = await dataApiRequest.json();

		return json({
			item: getItem,
			league: desiredLeague,
			records: records,
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
	const data = useLoaderData<typeof loader>();
	const chartDatasets = recordsToChartData(data);

	return (
		<AppShell
			padding="md"
			header={
				<Header height={120} p="md">
					{<HeaderWithSearch />}
				</Header>
			}
			navbar={
				<Navbar width={{ base: 300 }} height={500} p="md">
					{<SidebarWithFilters />}
				</Navbar>
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
							text: `${data.league} - ${data.item} Prices`,
						},
					},
				}}
				data={chartDatasets}
			/>
		</AppShell>
	);
}

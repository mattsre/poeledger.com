import { Flex, Select } from "@mantine/core";
import { Form, useLoaderData } from "@remix-run/react";
import Fuse from "fuse.js";
import { useState } from "react";



export default function HeaderWithSearch() {
	const data = useLoaderData();
	const fuseFilters = new Fuse(data.filters, { threshold: 0.2 });
	const fuseLeagues = new Fuse(data.leagues, { threshold: 0.5 });
	const [filters, setFilters] = useState<string[]>(data.filters)
	const [leagues, setLeagues] = useState<string[]>(data.leagues)

	function updateFilters(value: string) {
		setFilters(fuseFilters.search(value).map((item) => item.item) as string[]);
	}

	function updateLeagues(value: string) {
		setLeagues(fuseLeagues.search(value).map((item) => item.item) as string[]);
	}

	function filter(value: string, item: unknown) {
		return true;
	}

	return (
		<Flex justify="space-between" align="center">
			<h1>PoE Ledger</h1>

			<Form method="get">
				<Flex justify="flex-end" align="center" gap="md">
					<Select
						searchable
						onSearchChange={updateFilters}
						filter={filter}
						name="name"
						placeholder={data.currentItem}
						data={filters}
					/>
					<Select
						searchable
						onSearchChange={updateLeagues}
						filter={filter}
						name="league"
						placeholder={data.currentLeague}
						data={leagues}
					/>

					<input type="submit" hidden></input>
				</Flex>
			</Form>
		</Flex>
	);
}

import { Button, Container, Flex, Input, Select } from "@mantine/core";
import { json } from "@remix-run/node";
import { Form, useLoaderData } from "@remix-run/react";
import Fuse from "fuse.js";
import { useState } from "react";



export default function HeaderWithSearch() {
	const data = useLoaderData()
	const fuse = new Fuse(data.filters, { threshold: 0.3 });
	const [filters, setFilters] = useState<string[]>(data.filters)

	function updateSearch(value: string) {
		setFilters(fuse.search(value).map((item) => item.item) as string[]);
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
						nothingFound="Nothing Found"
						onSearchChange={updateSearch}
						filter={filter}
						name="get"
						placeholder={data.item}
						data={filters}
					/>
					<Select
						defaultValue={"Sanctum"}
						placeholder="Leagues"
						data={[{ value: "Sanctum", label: "Sanctum" }]}
					/>

					<input type="submit" hidden></input>
				</Flex>
			</Form>
		</Flex>
	);
}

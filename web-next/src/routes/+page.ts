import type { PageLoad } from "./$types";

import { PUBLIC_API_HOST } from "$env/static/public";

export const load: PageLoad = async ({ fetch, url }) => {
	const searchItemName = url.searchParams.get("item");

	if (!PUBLIC_API_HOST) {
		console.log("public api host not set");
		return null;
	}

	if (!searchItemName) {
		return null;
	}

	const response = await fetch(
		`${PUBLIC_API_HOST}/history?item=${searchItemName}`,
	);
	if (!response.ok) {
		return null;
	}

	const data = await response.json();

	return data;
};

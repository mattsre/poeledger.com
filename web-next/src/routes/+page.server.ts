import { PUBLIC_API_HOST } from "$env/static/public";

import type { PageServerLoad } from "./$types";
import { superValidate } from "sveltekit-superforms";
import { itemSearchFormSchema } from "$lib/components/item-search.svelte";
import { zod } from "sveltekit-superforms/adapters";
 
export const load: PageServerLoad = async () => {
  const fetchLeagues = async () => {
    const response = await fetch(`${PUBLIC_API_HOST}/leagues`);

    if (!response.ok) {
      console.error(response)
      return null;
    }

    return await response.json();
  };

  const fetchHeadhunterHistory = async () => {
    const response = await fetch(`${PUBLIC_API_HOST}/history?item=Headhunter`);

    if (!response.ok) {
      console.error(response)
    }

    return await response.json();
  }
  
  return {
    form: await superValidate(zod(itemSearchFormSchema)),
    initialPriceHistory: await fetchHeadhunterHistory(),
    leagues: await fetchLeagues(),
  };
};

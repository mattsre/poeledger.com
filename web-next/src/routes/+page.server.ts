import type { PageServerLoad } from "./$types";
import { superValidate } from "sveltekit-superforms";
import { itemSearchFormSchema } from "./schema";
import { zod } from "sveltekit-superforms/adapters";
 
export const load: PageServerLoad = async () => {
  let initialPriceHistory = null;
  const defaultHistoryUrl = "http://localhost:3000/history?item=Headhunter";
  
  const response = await fetch(defaultHistoryUrl);
  if (response.ok) {
    initialPriceHistory = await response.json();
  } else {
    console.error(`failed to request to ${defaultHistoryUrl}`)
    console.error(response)
  }

  return {
    form: await superValidate(zod(itemSearchFormSchema)),
    initialPriceHistory,
  };
};

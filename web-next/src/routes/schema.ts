import { z } from "zod";

export const itemSearchFormSchema = z.object({
  item: z.string(),
  intervalAmount: z.coerce.number().gt(0, "Interval must be greater than 0"),
  intervalUnit: z.string(),
  tenthQuantile: z.boolean().default(true),
  fifteenthQuantile: z.boolean().default(false).optional(),
  thirtiethQuantile: z.boolean().default(false).optional(),
  startTime: z.string(),
  endTime: z.string(),
})

export type ItemSearchFormSchema = typeof itemSearchFormSchema;

/*
 Generated by typeshare 1.7.0
*/

export enum League {
	Crucible = "Crucible",
	Sanctum = "Sanctum",
	Kalandra = "Kalandra",
	Sentinel = "Sentinel",
	Archnemesis = "Archnemesis",
	Scourge = "Scourge",
	Expedition = "Expedition",
	Ultimatum = "Ultimatum",
	Ritual = "Ritual",
	Heist = "Heist",
}

export enum Confidence {
	High = "High",
	Medium = "Medium",
	Low = "Low",
}

export enum ItemLinks {
	OneToFour = "1-4 links",
	Five = "5 links",
	Six = "6 links",
}

export interface PriceRecord {
	league: League;
	confidence: Confidence;
	date: Date;
	value: number;
	name: string;
	itemId?: number;
	itemType?: string;
	baseType?: string;
	itemVariant?: string;
	itemLinks?: ItemLinks;
}


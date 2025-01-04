export type AnotherTestObjectQueryParams = [];

export interface IAnotherTestObjectQueryResult {
	flavor_text: string | null;
	id: number;
	inventory_id: number | null;
	name: string;
	rarity: string | null;
	stats: object | null;
};

export interface IAnotherTestObjectQueryQuery {
	params: AnotherTestObjectQueryParams;
	result: IAnotherTestObjectQueryResult;
};

export type NestedTestObjectQueryParams = [];

export interface INestedTestObjectQueryResult {
	flavor_text: string | null;
	id: number;
	inventory_id: number | null;
	name: string;
	rarity: string | null;
	stats: object | null;
};

export interface INestedTestObjectQueryQuery {
	params: NestedTestObjectQueryParams;
	result: INestedTestObjectQueryResult;
};


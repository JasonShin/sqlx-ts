export type TestAsyncUsingParams = [];

export interface ITestAsyncUsingResult {
	flavor_text: string | null;
	id: number;
	inventory_id: number | null;
	name: string;
	rarity: string | null;
	stats: object | null;
};

export interface ITestAsyncUsingQuery {
	params: TestAsyncUsingParams;
	result: ITestAsyncUsingResult;
};


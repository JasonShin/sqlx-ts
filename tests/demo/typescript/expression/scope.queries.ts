export type TestParenthesisQueryParams = [];

export interface ITestParenthesisQueryResult {
	flavor_text: string | null;
	id: number;
	inventory_id: number | null;
	name: string;
	rarity: string | null;
	stats: object | null;
};

export interface ITestParenthesisQueryQuery {
	params: TestParenthesisQueryParams;
	result: ITestParenthesisQueryResult;
};

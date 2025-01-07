export type TestNullishCoalescingQueryParams = [];

export interface ITestNullishCoalescingQueryResult {
	flavor_text: string | null;
	id: number;
	inventory_id: number | null;
	name: string;
	rarity: string | null;
};

export interface ITestNullishCoalescingQueryQuery {
	params: TestNullishCoalescingQueryParams;
	result: ITestNullishCoalescingQueryResult;
};

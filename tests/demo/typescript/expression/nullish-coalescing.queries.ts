

export type TestNullishCoalescingQueryParams = [];


export interface ITestNullishCoalescingQueryResult {
    food_type: string;
	id: number;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
};


export interface ITestNullishCoalescingQueryQuery {
    params: TestNullishCoalescingQueryParams;
    result: ITestNullishCoalescingQueryResult;
};


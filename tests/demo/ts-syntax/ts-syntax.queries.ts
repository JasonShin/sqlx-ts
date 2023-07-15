

export type TestAwaitQueryParams = [];


export interface ITestAwaitQueryResult {
    food_type: string;
	id: number;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
};


export interface ITestAwaitQueryQuery {
    params: TestAwaitQueryParams;
    result: ITestAwaitQueryResult;
};


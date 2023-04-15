

export type TestQueryParams = [];


export interface ITestQueryResult {
    food_type: string;
	id: number;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
};


export interface ITestQueryQuery {
    params: TestQueryParams;
    result: ITestQueryResult;
};


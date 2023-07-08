

export type TestSequelizeQueryParams = [number];


export interface ITestSequelizeQueryResult {
    food_type: string;
	id: number;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
};


export interface ITestSequelizeQueryQuery {
    params: TestSequelizeQueryParams;
    result: ITestSequelizeQueryResult;
};


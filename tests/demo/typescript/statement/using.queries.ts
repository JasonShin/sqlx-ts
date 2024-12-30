export type TestAsyncUsingParams = [];

export interface ITestAsyncUsingResult {
	description: string | null;
	food_type: string;
	id: number;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
};

export interface ITestAsyncUsingQuery {
	params: TestAsyncUsingParams;
	result: ITestAsyncUsingResult;
};

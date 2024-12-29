

export type TestParenthesisQueryParams = [];


export interface ITestParenthesisQueryResult {
    description: string | null;
	food_type: string;
	id: number;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
};


export interface ITestParenthesisQueryQuery {
    params: TestParenthesisQueryParams;
    result: ITestParenthesisQueryResult;
};



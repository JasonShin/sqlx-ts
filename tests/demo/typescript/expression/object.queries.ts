export type AnotherTestObjectQueryParams = [];

export interface IAnotherTestObjectQueryResult {
	description: string | null;
	food_type: string;
	id: number;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
};

export interface IAnotherTestObjectQueryQuery {
	params: AnotherTestObjectQueryParams;
	result: IAnotherTestObjectQueryResult;
};

export type NestedTestObjectQueryParams = [];

export interface INestedTestObjectQueryResult {
	description: string | null;
	food_type: string;
	id: number;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
};

export interface INestedTestObjectQueryQuery {
	params: NestedTestObjectQueryParams;
	result: INestedTestObjectQueryResult;
};

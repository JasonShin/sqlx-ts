

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




export type ArrayQueryParams = [];


export interface IArrayQueryResult {
    food_type: string;
	id: number;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
};


export interface IArrayQueryQuery {
    params: ArrayQueryParams;
    result: IArrayQueryResult;
};




export type AnotherTestObjectQueryParams = [];


export interface IAnotherTestObjectQueryResult {
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


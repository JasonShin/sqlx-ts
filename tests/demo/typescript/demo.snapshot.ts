

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




export type TestAwaitQuery2Params = [];


export interface ITestAwaitQuery2Result {
    food_type: string;
	id: number;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
};


export interface ITestAwaitQuery2Query {
    params: TestAwaitQuery2Params;
    result: ITestAwaitQuery2Result;
};




export type AwaitClientQueryParams = [];


export interface IAwaitClientQueryResult {
    food_type: string;
	id: number;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
};


export interface IAwaitClientQueryQuery {
    params: AwaitClientQueryParams;
    result: IAwaitClientQueryResult;
};




export type GetItemsWithRowsParams = [];


export interface IGetItemsWithRowsResult {
    food_type: string;
	id: number;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
};


export interface IGetItemsWithRowsQuery {
    params: GetItemsWithRowsParams;
    result: IGetItemsWithRowsResult;
};




export type TestInsertParams = [[string, number]];


export interface ITestInsertResult {
    
};


export interface ITestInsertQuery {
    params: TestInsertParams;
    result: ITestInsertResult;
};



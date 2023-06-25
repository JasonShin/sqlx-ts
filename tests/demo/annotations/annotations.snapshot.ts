

export type TestMysqlQueryParams = [];


export interface ITestMysqlQueryResult {
    food_type: string;
	id: number;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
};


export interface ITestMysqlQueryQuery {
    params: TestMysqlQueryParams;
    result: ITestMysqlQueryResult;
};




export type TestPostgresQueryParams = [];


export interface ITestPostgresQueryResult {
    food_type: string;
	id: number;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
};


export interface ITestPostgresQueryQuery {
    params: TestPostgresQueryParams;
    result: ITestPostgresQueryResult;
};




export type Sql3Params = [];


export interface ISql3Result {
    points: string;
};


export interface ISql3Query {
    params: Sql3Params;
    result: ISql3Result;
};




export type Sql4Params = [string];


export interface ISql4Result {
    food_type: string;
	id: number;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
};


export interface ISql4Query {
    params: Sql4Params;
    result: ISql4Result;
};




export type Sql5Params = [string];


export interface ISql5Result {
    tableId: boolean;
};


export interface ISql5Query {
    params: Sql5Params;
    result: ISql5Result;
};



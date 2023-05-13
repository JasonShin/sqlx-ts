

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


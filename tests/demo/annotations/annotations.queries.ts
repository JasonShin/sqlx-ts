export type TestMysqlQueryParams = [];

export interface ITestMysqlQueryResult {
	description: string | null;
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
	description: string | null;
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

export type AnnotationSql3Params = [];

export interface IAnnotationSql3Result {
	points: string;
};

export interface IAnnotationSql3Query {
	params: AnnotationSql3Params;
	result: IAnnotationSql3Result;
};

export type AnnotationSql4Params = [string];

export interface IAnnotationSql4Result {
	description: string | null;
	food_type: string;
	id: number;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
};

export interface IAnnotationSql4Query {
	params: AnnotationSql4Params;
	result: IAnnotationSql4Result;
};

export type AnnotationSql5Params = [string];

export interface IAnnotationSql5Result {
	tableId: boolean;
};

export interface IAnnotationSql5Query {
	params: AnnotationSql5Params;
	result: IAnnotationSql5Result;
};

export type TestMysqlQueryWithParamOverridesParams = [string];

export interface ITestMysqlQueryWithParamOverridesResult {
	tableId: number;
};

export interface ITestMysqlQueryWithParamOverridesQuery {
	params: TestMysqlQueryWithParamOverridesParams;
	result: ITestMysqlQueryWithParamOverridesResult;
};

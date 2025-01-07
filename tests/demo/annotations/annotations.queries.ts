export type TestMysqlQueryParams = [];

export interface ITestMysqlQueryResult {
	flavor_text: string | null;
	id: number;
	inventory_id: number | null;
	name: string;
	rarity: string | null;
};

export interface ITestMysqlQueryQuery {
	params: TestMysqlQueryParams;
	result: ITestMysqlQueryResult;
};

export type TestPostgresQueryParams = [];

export interface ITestPostgresQueryResult {
	flavor_text: string | null;
	id: number;
	inventory_id: number | null;
	name: string;
	rarity: string | null;
};

export interface ITestPostgresQueryQuery {
	params: TestPostgresQueryParams;
	result: ITestPostgresQueryResult;
};

export type AnnotationSql3Params = [];

export interface IAnnotationSql3Result {
	id: string;
};

export interface IAnnotationSql3Query {
	params: AnnotationSql3Params;
	result: IAnnotationSql3Result;
};

export type AnnotationSql4Params = [string | null];

export interface IAnnotationSql4Result {
	character_id: number | null;
	id: number;
	quantity: number | null;
};

export interface IAnnotationSql4Query {
	params: AnnotationSql4Params;
	result: IAnnotationSql4Result;
};

export type AnnotationSql5Params = [string];

export interface IAnnotationSql5Result {
	inventoryId: boolean;
};

export interface IAnnotationSql5Query {
	params: AnnotationSql5Params;
	result: IAnnotationSql5Result;
};

export type TestMysqlQueryWithParamOverridesParams = [string];

export interface ITestMysqlQueryWithParamOverridesResult {
	inventoryId: number;
};

export interface ITestMysqlQueryWithParamOverridesQuery {
	params: TestMysqlQueryWithParamOverridesParams;
	result: ITestMysqlQueryWithParamOverridesResult;
};

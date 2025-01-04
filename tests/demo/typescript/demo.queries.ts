export type TestSequelizeQueryParams = [number];

export interface ITestSequelizeQueryResult {
	flavor_text: string | null;
	id: number;
	inventory_id: number | null;
	name: string;
	rarity: string | null;
	stats: object | null;
};

export interface ITestSequelizeQueryQuery {
	params: TestSequelizeQueryParams;
	result: ITestSequelizeQueryResult;
};

export type TestAwaitQueryParams = [];

export interface ITestAwaitQueryResult {
	flavor_text: string | null;
	id: number;
	inventory_id: number | null;
	name: string;
	rarity: string | null;
	stats: object | null;
};

export interface ITestAwaitQueryQuery {
	params: TestAwaitQueryParams;
	result: ITestAwaitQueryResult;
};

export type TestAwaitQuery2Params = [];

export interface ITestAwaitQuery2Result {
	flavor_text: string | null;
	id: number;
	inventory_id: number | null;
	name: string;
	rarity: string | null;
	stats: object | null;
};

export interface ITestAwaitQuery2Query {
	params: TestAwaitQuery2Params;
	result: ITestAwaitQuery2Result;
};

export type AwaitClientQueryParams = [];

export interface IAwaitClientQueryResult {
	flavor_text: string | null;
	id: number;
	inventory_id: number | null;
	name: string;
	rarity: string | null;
	stats: object | null;
};

export interface IAwaitClientQueryQuery {
	params: AwaitClientQueryParams;
	result: IAwaitClientQueryResult;
};

export type GetItemsWithRowsParams = [];

export interface IGetItemsWithRowsResult {
	flavor_text: string | null;
	id: number;
	inventory_id: number | null;
	name: string;
	rarity: string | null;
	stats: object | null;
};

export interface IGetItemsWithRowsQuery {
	params: GetItemsWithRowsParams;
	result: IGetItemsWithRowsResult;
};

export type TestInsertParams = [[number, string, string, object]];

export interface ITestInsertResult {
	
};

export interface ITestInsertQuery {
	params: TestInsertParams;
	result: ITestInsertResult;
};

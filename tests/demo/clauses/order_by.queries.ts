export type OrderByAscParams = [];

export interface IOrderByAscResult {
	id: number;
	name: string;
}

export interface IOrderByAscQuery {
	params: OrderByAscParams;
	result: IOrderByAscResult;
}

export type OrderByDescParams = [];

export interface IOrderByDescResult {
	id: number;
	name: string;
}

export interface IOrderByDescQuery {
	params: OrderByDescParams;
	result: IOrderByDescResult;
}

export type OrderByMultipleParams = [];

export interface IOrderByMultipleResult {
	id: number;
	name: string;
	rarity: string | null;
}

export interface IOrderByMultipleQuery {
	params: OrderByMultipleParams;
	result: IOrderByMultipleResult;
}

export type OrderByNullsFirstParams = [];

export interface IOrderByNullsFirstResult {
	id: number;
	name: string;
	rarity: string | null;
}

export interface IOrderByNullsFirstQuery {
	params: OrderByNullsFirstParams;
	result: IOrderByNullsFirstResult;
}

export type OrderByNullsLastParams = [];

export interface IOrderByNullsLastResult {
	id: number;
	name: string;
	rarity: string | null;
}

export interface IOrderByNullsLastQuery {
	params: OrderByNullsLastParams;
	result: IOrderByNullsLastResult;
}

export type OrderByExpressionParams = [];

export interface IOrderByExpressionResult {
	id: number;
	name: string;
}

export interface IOrderByExpressionQuery {
	params: OrderByExpressionParams;
	result: IOrderByExpressionResult;
}

export type OrderByWithParamsParams = [string | null];

export interface IOrderByWithParamsResult {
	id: number;
	name: string;
	rarity: string | null;
}

export interface IOrderByWithParamsQuery {
	params: OrderByWithParamsParams;
	result: IOrderByWithParamsResult;
}

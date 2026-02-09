export type ArrayAggBasicParams = [];

export interface IArrayAggBasicResult {
	names: unknown;
	rarity: string | null;
}

export interface IArrayAggBasicQuery {
	params: ArrayAggBasicParams;
	result: IArrayAggBasicResult;
}

export type ArrayAggWithOrderByParams = [];

export interface IArrayAggWithOrderByResult {
	namesOrdered: unknown;
	rarity: string | null;
}

export interface IArrayAggWithOrderByQuery {
	params: ArrayAggWithOrderByParams;
	result: IArrayAggWithOrderByResult;
}

export type ArrayLiteralUnknownParams = [];

export interface IArrayLiteralUnknownResult {
	id: number;
	name: string;
	rarity: string | null;
}

export interface IArrayLiteralUnknownQuery {
	params: ArrayLiteralUnknownParams;
	result: IArrayLiteralUnknownResult;
}

export type ArrayWithParamsParams = [Array<string>];

export interface IArrayWithParamsResult {
	id: number;
	name: string;
	rarity: string | null;
}

export interface IArrayWithParamsQuery {
	params: ArrayWithParamsParams;
	result: IArrayWithParamsResult;
}


export type ArrayAggBasicParams = [];

export interface IArrayAggBasicResult {
	names: any;
	rarity: string | null;
}

export interface IArrayAggBasicQuery {
	params: ArrayAggBasicParams;
	result: IArrayAggBasicResult;
}

export type ArrayAggWithOrderByParams = [];

export interface IArrayAggWithOrderByResult {
	namesOrdered: any;
	rarity: string | null;
}

export interface IArrayAggWithOrderByQuery {
	params: ArrayAggWithOrderByParams;
	result: IArrayAggWithOrderByResult;
}

export type ArrayLiteralAnyParams = [];

export interface IArrayLiteralAnyResult {
	id: number;
	name: string;
	rarity: string | null;
}

export interface IArrayLiteralAnyQuery {
	params: ArrayLiteralAnyParams;
	result: IArrayLiteralAnyResult;
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

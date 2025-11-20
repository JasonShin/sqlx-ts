export type DistinctBasicParams = [];

export interface IDistinctBasicResult {
	rarity: string | null;
}

export interface IDistinctBasicQuery {
	params: DistinctBasicParams;
	result: IDistinctBasicResult;
}

export type DistinctMultipleParams = [];

export interface IDistinctMultipleResult {
	name: string;
	rarity: string | null;
}

export interface IDistinctMultipleQuery {
	params: DistinctMultipleParams;
	result: IDistinctMultipleResult;
}

export type DistinctWithWhereParams = [];

export interface IDistinctWithWhereResult {
	rarity: string | null;
}

export interface IDistinctWithWhereQuery {
	params: DistinctWithWhereParams;
	result: IDistinctWithWhereResult;
}

export type DistinctWithParamsParams = [number];

export interface IDistinctWithParamsResult {
	rarity: string | null;
}

export interface IDistinctWithParamsQuery {
	params: DistinctWithParamsParams;
	result: IDistinctWithParamsResult;
}

export type DistinctOnParams = [];

export interface IDistinctOnResult {
	id: number;
	name: string;
	rarity: string | null;
}

export interface IDistinctOnQuery {
	params: DistinctOnParams;
	result: IDistinctOnResult;
}

export type DistinctWithOrderByParams = [];

export interface IDistinctWithOrderByResult {
	rarity: string | null;
}

export interface IDistinctWithOrderByQuery {
	params: DistinctWithOrderByParams;
	result: IDistinctWithOrderByResult;
}

export type DistinctWithAggregateParams = [];

export interface IDistinctWithAggregateResult {
	uniqueRarities: number;
}

export interface IDistinctWithAggregateQuery {
	params: DistinctWithAggregateParams;
	result: IDistinctWithAggregateResult;
}

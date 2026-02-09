export type SimpleCteParams = [];

export interface ISimpleCteResult {
	id: number;
	name: string;
}

export interface ISimpleCteQuery {
	params: SimpleCteParams;
	result: ISimpleCteResult;
}

export type RankWithCteParams = [];

export interface IRankWithCteResult {
	id: number;
	name: string;
	rk: number;
}

export interface IRankWithCteQuery {
	params: RankWithCteParams;
	result: IRankWithCteResult;
}

export type MultipleCtesParams = [];

export interface IMultipleCtesResult {
	id: number;
	name: string;
}

export interface IMultipleCtesQuery {
	params: MultipleCtesParams;
	result: IMultipleCtesResult;
}

export type CteWithWildcardParams = [];

export interface ICteWithWildcardResult {
	id: number;
	name: string;
}

export interface ICteWithWildcardQuery {
	params: CteWithWildcardParams;
	result: ICteWithWildcardResult;
}

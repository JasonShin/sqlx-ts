export type LikeBasicParams = [];

export interface ILikeBasicResult {
	id: number;
	name: string;
	rarity: string | null;
}

export interface ILikeBasicQuery {
	params: LikeBasicParams;
	result: ILikeBasicResult;
}

export type LikeWithParamParams = [string];

export interface ILikeWithParamResult {
	id: number;
	name: string;
	rarity: string | null;
}

export interface ILikeWithParamQuery {
	params: LikeWithParamParams;
	result: ILikeWithParamResult;
}

export type IlikeParams = [];

export interface IIlikeResult {
	id: number;
	name: string;
	rarity: string | null;
}

export interface IIlikeQuery {
	params: IlikeParams;
	result: IIlikeResult;
}

export type NotLikeParams = [];

export interface INotLikeResult {
	id: number;
	name: string;
	rarity: string | null;
}

export interface INotLikeQuery {
	params: NotLikeParams;
	result: INotLikeResult;
}

export type SimilarToParams = [];

export interface ISimilarToResult {
	id: number;
	name: string;
	rarity: string | null;
}

export interface ISimilarToQuery {
	params: SimilarToParams;
	result: ISimilarToResult;
}


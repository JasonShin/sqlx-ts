export type BasicRankParams = [];

export interface IBasicRankResult {
	id: number;
	name: string;
	rank: number;
	rarity: string | null;
}

export interface IBasicRankQuery {
	params: BasicRankParams;
	result: IBasicRankResult;
}

export type DenseRankParams = [];

export interface IDenseRankResult {
	denseRank: number;
	id: number;
	name: string;
	rarity: string | null;
}

export interface IDenseRankQuery {
	params: DenseRankParams;
	result: IDenseRankResult;
}

export type RankWithPartitionParams = [];

export interface IRankWithPartitionResult {
	id: number;
	name: string;
	rank: number;
	rarity: string | null;
}

export interface IRankWithPartitionQuery {
	params: RankWithPartitionParams;
	result: IRankWithPartitionResult;
}

export type MultipleRankingParams = [];

export interface IMultipleRankingResult {
	denseRank: number;
	id: number;
	name: string;
	rank: number;
	rarity: string | null;
	rowNum: number;
}

export interface IMultipleRankingQuery {
	params: MultipleRankingParams;
	result: IMultipleRankingResult;
}

export type NtileQuartilesParams = [];

export interface INtileQuartilesResult {
	id: number;
	name: string;
	quartile: number;
}

export interface INtileQuartilesQuery {
	params: NtileQuartilesParams;
	result: INtileQuartilesResult;
}

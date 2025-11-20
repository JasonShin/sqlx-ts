export type SinglePartitionParams = [];

export interface ISinglePartitionResult {
	id: number;
	name: string;
	rarity: string | null;
	rarityCount: number;
}

export interface ISinglePartitionQuery {
	params: SinglePartitionParams;
	result: ISinglePartitionResult;
}

export type MultiplePartitionParams = [];

export interface IMultiplePartitionResult {
	countPerGroup: number;
	id: number;
	name: string;
	rarity: string | null;
}

export interface IMultiplePartitionQuery {
	params: MultiplePartitionParams;
	result: IMultiplePartitionResult;
}

export type WindowAggregatesParams = [];

export interface IWindowAggregatesResult {
	avgId: number;
	count: number;
	id: number;
	maxId: number;
	minId: number;
	name: string;
	rarity: string | null;
	sumId: number;
}

export interface IWindowAggregatesQuery {
	params: WindowAggregatesParams;
	result: IWindowAggregatesResult;
}

export type WindowWithOrderByParams = [];

export interface IWindowWithOrderByResult {
	id: number;
	name: string;
	rarity: string | null;
	runningSum: number;
}

export interface IWindowWithOrderByQuery {
	params: WindowWithOrderByParams;
	result: IWindowWithOrderByResult;
}

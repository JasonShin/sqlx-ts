export type GroupBySingleParams = [];

export interface IGroupBySingleResult {
	count: number;
	rarity: string | null;
}

export interface IGroupBySingleQuery {
	params: GroupBySingleParams;
	result: IGroupBySingleResult;
}

export type GroupByMultipleParams = [];

export interface IGroupByMultipleResult {
	count: number;
	name: string;
	rarity: string | null;
}

export interface IGroupByMultipleQuery {
	params: GroupByMultipleParams;
	result: IGroupByMultipleResult;
}

export type GroupByWithAggregatesParams = [];

export interface IGroupByWithAggregatesResult {
	maxId: number;
	minId: number;
	rarity: string | null;
	totalCount: number;
	withInventory: number;
}

export interface IGroupByWithAggregatesQuery {
	params: GroupByWithAggregatesParams;
	result: IGroupByWithAggregatesResult;
}

export type GroupByWithWhereParams = [];

export interface IGroupByWithWhereResult {
	count: number;
	rarity: string | null;
}

export interface IGroupByWithWhereQuery {
	params: GroupByWithWhereParams;
	result: IGroupByWithWhereResult;
}

export type GroupByWithParamsParams = [string | null];

export interface IGroupByWithParamsResult {
	count: number;
	rarity: string | null;
}

export interface IGroupByWithParamsQuery {
	params: GroupByWithParamsParams;
	result: IGroupByWithParamsResult;
}

export type GroupByWithJoinParams = [];

export interface IGroupByWithJoinResult {
	itemsRarity: string | null;
	totalQuantity: number;
}

export interface IGroupByWithJoinQuery {
	params: GroupByWithJoinParams;
	result: IGroupByWithJoinResult;
}

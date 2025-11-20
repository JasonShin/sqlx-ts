export type CaseInWhereParams = [];

export interface ICaseInWhereResult {
	id: number;
	name: string;
	rarity: string | null;
}

export interface ICaseInWhereQuery {
	params: CaseInWhereParams;
	result: ICaseInWhereResult;
}

export type CaseInOrderByParams = [];

export interface ICaseInOrderByResult {
	id: number;
	name: string;
	rarity: string | null;
}

export interface ICaseInOrderByQuery {
	params: CaseInOrderByParams;
	result: ICaseInOrderByResult;
}

export type CaseInGroupByHavingParams = [];

export interface ICaseInGroupByHavingResult {
	count: number;
	rarityGroup: any;
}

export interface ICaseInGroupByHavingQuery {
	params: CaseInGroupByHavingParams;
	result: ICaseInGroupByHavingResult;
}

export type CaseWithAggregatesParams = [];

export interface ICaseWithAggregatesResult {
	countAbove5: number;
	countAtOrBelow5: number;
	rarity: string | null;
}

export interface ICaseWithAggregatesQuery {
	params: CaseWithAggregatesParams;
	result: ICaseWithAggregatesResult;
}


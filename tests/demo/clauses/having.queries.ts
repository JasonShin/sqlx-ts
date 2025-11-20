export type HavingBasicParams = [];

export interface IHavingBasicResult {
	count: number;
	rarity: string | null;
}

export interface IHavingBasicQuery {
	params: HavingBasicParams;
	result: IHavingBasicResult;
}

export type HavingWithAggregateParams = [];

export interface IHavingWithAggregateResult {
	count: number;
	maxId: number;
	rarity: string | null;
}

export interface IHavingWithAggregateQuery {
	params: HavingWithAggregateParams;
	result: IHavingWithAggregateResult;
}

export type HavingWithSumParams = [];

export interface IHavingWithSumResult {
	itemsRarity: string | null;
	totalQuantity: number;
}

export interface IHavingWithSumQuery {
	params: HavingWithSumParams;
	result: IHavingWithSumResult;
}

export type HavingMultipleConditionsParams = [];

export interface IHavingMultipleConditionsResult {
	avgId: number;
	count: number;
	rarity: string | null;
}

export interface IHavingMultipleConditionsQuery {
	params: HavingMultipleConditionsParams;
	result: IHavingMultipleConditionsResult;
}

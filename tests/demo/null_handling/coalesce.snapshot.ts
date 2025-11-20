export type CoalesceBasicParams = [];

export interface ICoalesceBasicResult {
	id: number;
	name: string;
	rarityDisplay: string;
}

export interface ICoalesceBasicQuery {
	params: CoalesceBasicParams;
	result: ICoalesceBasicResult;
}

export type CoalesceMultipleParams = [];

export interface ICoalesceMultipleResult {
	description: string;
	id: number;
	name: string;
}

export interface ICoalesceMultipleQuery {
	params: CoalesceMultipleParams;
	result: ICoalesceMultipleResult;
}

export type CoalesceNumericParams = [];

export interface ICoalesceNumericResult {
	id: number;
	inventoryIdSafe: number;
	name: string;
}

export interface ICoalesceNumericQuery {
	params: CoalesceNumericParams;
	result: ICoalesceNumericResult;
}

export type CoalesceWithAggregatesParams = [];

export interface ICoalesceWithAggregatesResult {
	count: number;
	rarityGroup: string;
}

export interface ICoalesceWithAggregatesQuery {
	params: CoalesceWithAggregatesParams;
	result: ICoalesceWithAggregatesResult;
}


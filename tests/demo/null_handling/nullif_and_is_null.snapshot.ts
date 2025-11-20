export type IsNullBasicParams = [];

export interface IIsNullBasicResult {
	id: number;
	name: string;
	rarity: string | null;
}

export interface IIsNullBasicQuery {
	params: IsNullBasicParams;
	result: IIsNullBasicResult;
}

export type IsNotNullParams = [];

export interface IIsNotNullResult {
	id: number;
	name: string;
	rarity: string | null;
}

export interface IIsNotNullQuery {
	params: IsNotNullParams;
	result: IIsNotNullResult;
}

export type NullifBasicParams = [];

export interface INullifBasicResult {
	id: number;
	name: string;
	rarityExcludingCommon: string;
}

export interface INullifBasicQuery {
	params: NullifBasicParams;
	result: INullifBasicResult;
}

export type NullifEmptyStringParams = [];

export interface INullifEmptyStringResult {
	id: number;
	name: string;
	nameSafe: string;
}

export interface INullifEmptyStringQuery {
	params: NullifEmptyStringParams;
	result: INullifEmptyStringResult;
}

export type IsDistinctFromParams = [string];

export interface IIsDistinctFromResult {
	id: number;
	name: string;
	rarity: string | null;
}

export interface IIsDistinctFromQuery {
	params: IsDistinctFromParams;
	result: IIsDistinctFromResult;
}

export type IsNotDistinctFromParams = [string];

export interface IIsNotDistinctFromResult {
	id: number;
	name: string;
	rarity: string | null;
}

export interface IIsNotDistinctFromQuery {
	params: IsNotDistinctFromParams;
	result: IIsNotDistinctFromResult;
}


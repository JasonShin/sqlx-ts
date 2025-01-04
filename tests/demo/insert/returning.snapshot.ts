export type InsertWildcardParams = [];

export interface IInsertWildcardResult {
	flavorText: string | null;
	id: number;
	inventoryId: number | null;
	name: string;
	rarity: string | null;
	stats: object | null;
};

export interface IInsertWildcardQuery {
	params: InsertWildcardParams;
	result: IInsertWildcardResult;
};

export type InsertSelectorParams = [];

export interface IInsertSelectorResult {
	id: number;
	rarity: string | null;
};

export interface IInsertSelectorQuery {
	params: InsertSelectorParams;
	result: IInsertSelectorResult;
};

export type InsertAliasParams = [];

export interface IInsertAliasResult {
	id1: number;
	rarity1: string | null;
};

export interface IInsertAliasQuery {
	params: InsertAliasParams;
	result: IInsertAliasResult;
};

export type InsertQuotedParams = [];

export interface IInsertQuotedResult {
	id1: number;
	rarity1: string | null;
};

export interface IInsertQuotedQuery {
	params: InsertQuotedParams;
	result: IInsertQuotedResult;
};

export type InsertParamsParams = [];

export interface IInsertParamsResult {
	id1: number;
	rarity1: string | null;
};

export interface IInsertParamsQuery {
	params: InsertParamsParams;
	result: IInsertParamsResult;
};


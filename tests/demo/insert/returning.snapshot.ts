

export type InsertWildcardParams = [];


export interface IInsertWildcardResult {
    description: string;
	foodType: string;
	id: number;
	points: number;
	tableId: number;
	timeTakesToCook: number;
};


export interface IInsertWildcardQuery {
    params: InsertWildcardParams;
    result: IInsertWildcardResult;
};




export type InsertSelectorParams = [];


export interface IInsertSelectorResult {
    foodType: string;
	id: number;
};


export interface IInsertSelectorQuery {
    params: InsertSelectorParams;
    result: IInsertSelectorResult;
};




export type InsertAliasParams = [];


export interface IInsertAliasResult {
    foodType1: string;
	id1: number;
};


export interface IInsertAliasQuery {
    params: InsertAliasParams;
    result: IInsertAliasResult;
};




export type InsertQuotedParams = [];


export interface IInsertQuotedResult {
    foodType1: string;
	id1: number;
};


export interface IInsertQuotedQuery {
    params: InsertQuotedParams;
    result: IInsertQuotedResult;
};




export type InsertParamsParams = [string, string | null];


export interface IInsertParamsResult {
    foodType1: string;
	id1: number;
};


export interface IInsertParamsQuery {
    params: InsertParamsParams;
    result: IInsertParamsResult;
};





export type WildcardParams = [];


export interface IWildcardResult {
    foodType: string;
	id: number;
	points: number;
	tableId: number;
	timeTakesToCook: number;
};


export interface IWildcardQuery {
    params: WildcardParams;
    result: IWildcardResult;
};




export type SelectorParams = [];


export interface ISelectorResult {
    foodType: string;
	id: number;
};


export interface ISelectorQuery {
    params: SelectorParams;
    result: ISelectorResult;
};




export type AliasParams = [];


export interface IAliasResult {
    foodType1: string;
	id1: number;
};


export interface IAliasQuery {
    params: AliasParams;
    result: IAliasResult;
};




export type QuotedParams = [];


export interface IQuotedResult {
    foodType1: string;
	id1: number;
};


export interface IQuotedQuery {
    params: QuotedParams;
    result: IQuotedResult;
};



export type SearchedCaseBasicParams = [];

export interface ISearchedCaseBasicResult {
	id: number;
	idCategory: any;
	name: string;
}

export interface ISearchedCaseBasicQuery {
	params: SearchedCaseBasicParams;
	result: ISearchedCaseBasicResult;
}

export type SearchedCaseMultipleConditionsParams = [];

export interface ISearchedCaseMultipleConditionsResult {
	id: number;
	itemClass: any;
	name: string;
	rarity: string | null;
}

export interface ISearchedCaseMultipleConditionsQuery {
	params: SearchedCaseMultipleConditionsParams;
	result: ISearchedCaseMultipleConditionsResult;
}

export type SearchedCaseWithParamsParams = [];

export interface ISearchedCaseWithParamsResult {
	id: number;
	name: string;
	thresholdStatus: any;
}

export interface ISearchedCaseWithParamsQuery {
	params: SearchedCaseWithParamsParams;
	result: ISearchedCaseWithParamsResult;
}


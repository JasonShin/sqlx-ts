export type JsonObjectBasicParams = [];

export interface IJsonObjectBasicResult {
	id: number;
	itemJson: any;
}

export interface IJsonObjectBasicQuery {
	params: JsonObjectBasicParams;
	result: IJsonObjectBasicResult;
}

export type JsonArrayAggregationParams = [];

export interface IJsonArrayAggregationResult {
	items: any;
	rarity: string | null;
}

export interface IJsonArrayAggregationQuery {
	params: JsonArrayAggregationParams;
	result: IJsonArrayAggregationResult;
}

export type JsonOperatorsSelectParams = [];

export interface IJsonOperatorsSelectResult {
	extractedName: any;
	id: number;
	name: string;
}

export interface IJsonOperatorsSelectQuery {
	params: JsonOperatorsSelectParams;
	result: IJsonOperatorsSelectResult;
}


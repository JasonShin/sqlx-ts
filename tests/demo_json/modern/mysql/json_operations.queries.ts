export type JsonArrayAggregationParams = [];

export interface IJsonArrayAggregationResult {
	items: any;
	rarity: string | null;
}

export interface IJsonArrayAggregationQuery {
	params: JsonArrayAggregationParams;
	result: IJsonArrayAggregationResult;
}

export type JsonObjectBasicParams = [];

export interface IJsonObjectBasicResult {
	id: number;
	itemJson: any;
}

export interface IJsonObjectBasicQuery {
	params: JsonObjectBasicParams;
	result: IJsonObjectBasicResult;
}

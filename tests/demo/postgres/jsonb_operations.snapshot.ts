export type JsonbBuildObjectBasicParams = [];

export interface IJsonbBuildObjectBasicResult {
	id: number;
	itemJson: any;
}

export interface IJsonbBuildObjectBasicQuery {
	params: JsonbBuildObjectBasicParams;
	result: IJsonbBuildObjectBasicResult;
}

export type JsonbAggregationParams = [];

export interface IJsonbAggregationResult {
	items: any;
	rarity: string | null;
}

export interface IJsonbAggregationQuery {
	params: JsonbAggregationParams;
	result: IJsonbAggregationResult;
}

export type JsonOperatorsSelectParams = [];

export interface IJsonOperatorsSelectResult {
	extractedName: string;
	id: number;
	name: string;
}

export interface IJsonOperatorsSelectQuery {
	params: JsonOperatorsSelectParams;
	result: IJsonOperatorsSelectResult;
}


export type JsonbArrayLengthParams = [];

export interface IJsonbArrayLengthResult {
	itemsCount: unknown;
	jsonTestDataId: number;
	jsonTestDataName: string;
	tagsCount: unknown;
}

export interface IJsonbArrayLengthQuery {
	params: JsonbArrayLengthParams;
	result: IJsonbArrayLengthResult;
}

export type JsonbArrayContainsParams = [];

export interface IJsonbArrayContainsResult {
	jsonTestDataId: number;
	jsonTestDataName: string;
	tags: string;
}

export interface IJsonbArrayContainsQuery {
	params: JsonbArrayContainsParams;
	result: IJsonbArrayContainsResult;
}


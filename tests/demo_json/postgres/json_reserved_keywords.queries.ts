export type JsonbReservedKeywordsParams = [];

export interface IJsonbReservedKeywordsResult {
	jsonTestDataId: number;
	jsonTestDataName: string;
	reservedKeywordsObject: { "class": string; "interface": string; "type": string; "const": string; "let": string; "function": string; "return": boolean; "import": string; "export": string; "async": string };
}

export interface IJsonbReservedKeywordsQuery {
	params: JsonbReservedKeywordsParams;
	result: IJsonbReservedKeywordsResult;
}

export type JsonbInvalidIdentifiersParams = [];

export interface IJsonbInvalidIdentifiersResult {
	invalidIdentifiersObject: { "field-name": string; "field name": string; "123field": string; "user@email": string; "field.nested": string };
	jsonTestDataId: number;
	jsonTestDataName: string;
}

export interface IJsonbInvalidIdentifiersQuery {
	params: JsonbInvalidIdentifiersParams;
	result: IJsonbInvalidIdentifiersResult;
}

export type JsonbAggReservedKeywordsParams = [];

export interface IJsonbAggReservedKeywordsResult {
	aggregatedReservedKeywords: Array<{ "class": string; "interface": number; "default": boolean }>;
}

export interface IJsonbAggReservedKeywordsQuery {
	params: JsonbAggReservedKeywordsParams;
	result: IJsonbAggReservedKeywordsResult;
}

export type JsonbMixedIdentifiersParams = [];

export interface IJsonbMixedIdentifiersResult {
	jsonTestDataId: number;
	mixedIdentifiersObject: { validName: string; "invalid-name": number; _underscore: string; $dollar: string; "class": string; "123start": string };
}

export interface IJsonbMixedIdentifiersQuery {
	params: JsonbMixedIdentifiersParams;
	result: IJsonbMixedIdentifiersResult;
}

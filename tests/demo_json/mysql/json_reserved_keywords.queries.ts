export type JsonReservedKeywordsParams = [];

export interface IJsonReservedKeywordsResult {
	id: number;
	name: string;
	reservedKeywordsObject: { "class": string; "interface": string; "type": string; "const": string; "let": string; "function": string; "return": boolean; "import": string; "export": string; "async": string };
}

export interface IJsonReservedKeywordsQuery {
	params: JsonReservedKeywordsParams;
	result: IJsonReservedKeywordsResult;
}

export type JsonInvalidIdentifiersParams = [];

export interface IJsonInvalidIdentifiersResult {
	id: number;
	invalidIdentifiersObject: { "field-name": string; "field name": string; "123field": string; "user@email": string; "field.nested": string };
	name: string;
}

export interface IJsonInvalidIdentifiersQuery {
	params: JsonInvalidIdentifiersParams;
	result: IJsonInvalidIdentifiersResult;
}

export type JsonMixedIdentifiersParams = [];

export interface IJsonMixedIdentifiersResult {
	id: number;
	mixedIdentifiersObject: { validName: string; "invalid-name": number; _underscore: string; $dollar: string; "class": string; "123start": string };
}

export interface IJsonMixedIdentifiersQuery {
	params: JsonMixedIdentifiersParams;
	result: IJsonMixedIdentifiersResult;
}

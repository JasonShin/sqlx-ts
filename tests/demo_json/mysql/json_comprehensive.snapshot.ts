export type JsonExtractParams = [];

export interface IJsonExtractResult {
	age: number;
	email: any;
	id: number;
	name: string;
	username: any;
}

export interface IJsonExtractQuery {
	params: JsonExtractParams;
	result: IJsonExtractResult;
}

export type JsonExtractShorthandParams = [];

export interface IJsonExtractShorthandResult {
	email: string;
	id: number;
	name: string;
	username: string;
}

export interface IJsonExtractShorthandQuery {
	params: JsonExtractShorthandParams;
	result: IJsonExtractShorthandResult;
}

export type JsonNestedPathParams = [];

export interface IJsonNestedPathResult {
	city: any;
	id: number;
	name: string;
	zipCode: any;
}

export interface IJsonNestedPathQuery {
	params: JsonNestedPathParams;
	result: IJsonNestedPathResult;
}

export type JsonArrayIndexParams = [];

export interface IJsonArrayIndexResult {
	firstItemName: any;
	firstItemPrice: number;
	id: number;
	name: string;
}

export interface IJsonArrayIndexQuery {
	params: JsonArrayIndexParams;
	result: IJsonArrayIndexResult;
}

export type JsonArrayLengthParams = [];

export interface IJsonArrayLengthResult {
	id: number;
	name: string;
	tagsCount: any;
}

export interface IJsonArrayLengthQuery {
	params: JsonArrayLengthParams;
	result: IJsonArrayLengthResult;
}

export type JsonTypeParams = [];

export interface IJsonTypeResult {
	ageType: any;
	id: number;
	tagsType: any;
	usernameType: any;
}

export interface IJsonTypeQuery {
	params: JsonTypeParams;
	result: IJsonTypeResult;
}

export type JsonContainsParams = [];

export interface IJsonContainsResult {
	id: number;
	isActive: any;
	name: string;
}

export interface IJsonContainsQuery {
	params: JsonContainsParams;
	result: IJsonContainsResult;
}

export type JsonKeysParams = [];

export interface IJsonKeysResult {
	allKeys: any;
	id: number;
	name: string;
}

export interface IJsonKeysQuery {
	params: JsonKeysParams;
	result: IJsonKeysResult;
}

export type JsonObjectBuildParams = [];

export interface IJsonObjectBuildResult {
	id: number;
	name: string;
	userSummary: { id: number; name: string; username: any; email: any };
}

export interface IJsonObjectBuildQuery {
	params: JsonObjectBuildParams;
	result: IJsonObjectBuildResult;
}

export type JsonFilterParams = [];

export interface IJsonFilterResult {
	id: number;
	name: string;
	username: any;
}

export interface IJsonFilterQuery {
	params: JsonFilterParams;
	result: IJsonFilterResult;
}

export type JsonDeepPathParams = [];

export interface IJsonDeepPathResult {
	appName: any;
	dbHost: any;
	dbPort: number;
	id: number;
}

export interface IJsonDeepPathQuery {
	params: JsonDeepPathParams;
	result: IJsonDeepPathResult;
}

export type JsonValidParams = [];

export interface IJsonValidResult {
	id: number;
	isValidJson: any;
	name: string;
}

export interface IJsonValidQuery {
	params: JsonValidParams;
	result: IJsonValidResult;
}

export type JsonSearchParams = [];

export interface IJsonSearchResult {
	id: number;
	name: string;
	usernamePath: any;
}

export interface IJsonSearchQuery {
	params: JsonSearchParams;
	result: IJsonSearchResult;
}


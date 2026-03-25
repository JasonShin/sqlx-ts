export type JsonAccessOperatorsParams = [];

export interface IJsonAccessOperatorsResult {
	age: number;
	email: string;
	id: number;
	name: string;
	username: string;
}

export interface IJsonAccessOperatorsQuery {
	params: JsonAccessOperatorsParams;
	result: IJsonAccessOperatorsResult;
}

export type JsonNestedAccessParams = [];

export interface IJsonNestedAccessResult {
	city: string;
	id: number;
	name: string;
	zipCode: string;
}

export interface IJsonNestedAccessQuery {
	params: JsonNestedAccessParams;
	result: IJsonNestedAccessResult;
}

export type JsonArrayIndexParams = [];

export interface IJsonArrayIndexResult {
	firstItemName: string;
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
	tagsCount: unknown;
}

export interface IJsonArrayLengthQuery {
	params: JsonArrayLengthParams;
	result: IJsonArrayLengthResult;
}

export type JsonTypeofParams = [];

export interface IJsonTypeofResult {
	ageType: unknown;
	id: number;
	tagsType: unknown;
	usernameType: unknown;
}

export interface IJsonTypeofQuery {
	params: JsonTypeofParams;
	result: IJsonTypeofResult;
}

export type JsonKeyExistsParams = [];

export interface IJsonKeyExistsResult {
	hasAddress: string;
	hasUsername: string;
	id: number;
	name: string;
}

export interface IJsonKeyExistsQuery {
	params: JsonKeyExistsParams;
	result: IJsonKeyExistsResult;
}

export type JsonContainsParams = [];

export interface IJsonContainsResult {
	id: number;
	isActive: string;
	name: string;
}

export interface IJsonContainsQuery {
	params: JsonContainsParams;
	result: IJsonContainsResult;
}

export type JsonBuildObjectTypedParams = [];

export interface IJsonBuildObjectTypedResult {
	id: number;
	name: string;
	userSummary: { id: number; name: string; username: any; email: any };
}

export interface IJsonBuildObjectTypedQuery {
	params: JsonBuildObjectTypedParams;
	result: IJsonBuildObjectTypedResult;
}

export type JsonFilterParams = [];

export interface IJsonFilterResult {
	id: number;
	name: string;
	username: string;
}

export interface IJsonFilterQuery {
	params: JsonFilterParams;
	result: IJsonFilterResult;
}

export type JsonDeepPathParams = [];

export interface IJsonDeepPathResult {
	appName: string;
	dbHost: string;
	dbPort: number;
	id: number;
}

export interface IJsonDeepPathQuery {
	params: JsonDeepPathParams;
	result: IJsonDeepPathResult;
}


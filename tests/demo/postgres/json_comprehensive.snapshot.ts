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
	zip_code: string;
}

export interface IJsonNestedAccessQuery {
	params: JsonNestedAccessParams;
	result: IJsonNestedAccessResult;
}

export type JsonArrayIndexParams = [];

export interface IJsonArrayIndexResult {
	first_item_name: string;
	first_item_price: number;
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
	tags_count: any;
}

export interface IJsonArrayLengthQuery {
	params: JsonArrayLengthParams;
	result: IJsonArrayLengthResult;
}

export type JsonTypeofParams = [];

export interface IJsonTypeofResult {
	age_type: any;
	id: number;
	tags_type: any;
	username_type: any;
}

export interface IJsonTypeofQuery {
	params: JsonTypeofParams;
	result: IJsonTypeofResult;
}

export type JsonKeyExistsParams = [];

export interface IJsonKeyExistsResult {
	has_address: string;
	has_username: string;
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
	is_active: string;
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
	user_summary: { id: number; name: string; username: any; email: any };
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
	app_name: string;
	db_host: string;
	db_port: number;
	id: number;
}

export interface IJsonDeepPathQuery {
	params: JsonDeepPathParams;
	result: IJsonDeepPathResult;
}

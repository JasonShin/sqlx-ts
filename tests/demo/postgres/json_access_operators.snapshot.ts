export type JsonFieldAccessParams = [];

export interface IJsonFieldAccessResult {
	active_json: string;
	age_json: string;
	id: number;
	name: string;
	username_json: string;
}

export interface IJsonFieldAccessQuery {
	params: JsonFieldAccessParams;
	result: IJsonFieldAccessResult;
}

export type JsonFieldAccessTextParams = [];

export interface IJsonFieldAccessTextResult {
	active: boolean;
	age: number;
	email: string;
	id: number;
	name: string;
	username: string;
}

export interface IJsonFieldAccessTextQuery {
	params: JsonFieldAccessTextParams;
	result: IJsonFieldAccessTextResult;
}

export type JsonNestedAccessParams = [];

export interface IJsonNestedAccessResult {
	address_json: string;
	city: string;
	city_json: string;
	id: number;
	name: string;
	street: string;
	street_json: string;
	zip_code: string;
}

export interface IJsonNestedAccessQuery {
	params: JsonNestedAccessParams;
	result: IJsonNestedAccessResult;
}

export type JsonArrayAccessParams = [];

export interface IJsonArrayAccessResult {
	first_item_json: number;
	first_item_name: string;
	first_item_price: string;
	id: number;
	items_json: string;
	name: string;
	second_item_json: number;
}

export interface IJsonArrayAccessQuery {
	params: JsonArrayAccessParams;
	result: IJsonArrayAccessResult;
}

export type JsonPathAccessParams = [];

export interface IJsonPathAccessResult {
	first_item_json: string;
	first_item_name: string;
	first_item_rarity: string;
	id: number;
	level: string;
	level_json: string;
	name: string;
}

export interface IJsonPathAccessQuery {
	params: JsonPathAccessParams;
	result: IJsonPathAccessResult;
}

export type JsonDeepPathAccessParams = [];

export interface IJsonDeepPathAccessResult {
	dark_mode: string;
	db_host: string;
	db_host_json: string;
	db_port: string;
	email_notifications: string;
	id: number;
	name: string;
}

export interface IJsonDeepPathAccessQuery {
	params: JsonDeepPathAccessParams;
	result: IJsonDeepPathAccessResult;
}

export type JsonFilterByFieldParams = [];

export interface IJsonFilterByFieldResult {
	email: string;
	id: number;
	name: string;
	username: string;
}

export interface IJsonFilterByFieldQuery {
	params: JsonFilterByFieldParams;
	result: IJsonFilterByFieldResult;
}

export type JsonNullHandlingParams = [];

export interface IJsonNullHandlingResult {
	first_comment: string;
	first_reviewer: string;
	id: number;
	second_comment: string;
	third_comment: string;
	third_reviewer: string;
}

export interface IJsonNullHandlingQuery {
	params: JsonNullHandlingParams;
	result: IJsonNullHandlingResult;
}

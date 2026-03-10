export type JsonFieldAccessParams = [];

export interface IJsonFieldAccessResult {
	activeJson: string;
	ageJson: string;
	id: number;
	name: string;
	usernameJson: string;
}

export interface IJsonFieldAccessQuery {
	params: JsonFieldAccessParams;
	result: IJsonFieldAccessResult;
}

export type JsonFieldAccessTextParams = [];

export interface IJsonFieldAccessTextResult {
	active: number;
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
	addressJson: string;
	city: any;
	cityJson: string;
	id: number;
	name: string;
	street: any;
	zipCode: any;
}

export interface IJsonNestedAccessQuery {
	params: JsonNestedAccessParams;
	result: IJsonNestedAccessResult;
}

export type JsonArrayAccessParams = [];

export interface IJsonArrayAccessResult {
	firstItemJson: string;
	firstItemName: any;
	firstItemPrice: number;
	id: number;
	itemsJson: string;
	name: string;
	secondItemJson: string;
}

export interface IJsonArrayAccessQuery {
	params: JsonArrayAccessParams;
	result: IJsonArrayAccessResult;
}

export type JsonPathAccessParams = [];

export interface IJsonPathAccessResult {
	firstItemJson: string;
	firstItemName: any;
	firstItemRarity: any;
	id: number;
	level: number;
	levelJson: string;
	name: string;
}

export interface IJsonPathAccessQuery {
	params: JsonPathAccessParams;
	result: IJsonPathAccessResult;
}

export type JsonDeepPathAccessParams = [];

export interface IJsonDeepPathAccessResult {
	darkMode: number;
	dbHost: any;
	dbHostJson: string;
	dbPort: number;
	emailNotifications: number;
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
	firstComment: any;
	firstReviewer: any;
	id: number;
	secondComment: any;
	thirdComment: any;
	thirdReviewer: any;
}

export interface IJsonNullHandlingQuery {
	params: JsonNullHandlingParams;
	result: IJsonNullHandlingResult;
}


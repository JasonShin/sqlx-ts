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
	addressJson: string;
	city: string;
	cityJson: string;
	id: number;
	name: string;
	street: string;
	streetJson: string;
	zipCode: string;
}

export interface IJsonNestedAccessQuery {
	params: JsonNestedAccessParams;
	result: IJsonNestedAccessResult;
}

export type JsonArrayAccessParams = [];

export interface IJsonArrayAccessResult {
	firstItemJson: number;
	firstItemName: string;
	firstItemPrice: string;
	id: number;
	itemsJson: string;
	name: string;
	secondItemJson: number;
}

export interface IJsonArrayAccessQuery {
	params: JsonArrayAccessParams;
	result: IJsonArrayAccessResult;
}

export type JsonPathAccessParams = [];

export interface IJsonPathAccessResult {
	firstItemJson: string;
	firstItemName: string;
	firstItemRarity: string;
	id: number;
	level: string;
	levelJson: string;
	name: string;
}

export interface IJsonPathAccessQuery {
	params: JsonPathAccessParams;
	result: IJsonPathAccessResult;
}

export type JsonDeepPathAccessParams = [];

export interface IJsonDeepPathAccessResult {
	darkMode: string;
	dbHost: string;
	dbHostJson: string;
	dbPort: string;
	emailNotifications: string;
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
	firstComment: string;
	firstReviewer: string;
	id: number;
	secondComment: string;
	thirdComment: string;
	thirdReviewer: string;
}

export interface IJsonNullHandlingQuery {
	params: JsonNullHandlingParams;
	result: IJsonNullHandlingResult;
}

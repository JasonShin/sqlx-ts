export type JsonArrayLengthParams = [];

export interface IJsonArrayLengthResult {
	id: number;
	itemsCount: unknown;
	name: string;
	tagsCount: unknown;
}

export interface IJsonArrayLengthQuery {
	params: JsonArrayLengthParams;
	result: IJsonArrayLengthResult;
}

export type JsonArrayExtractParams = [];

export interface IJsonArrayExtractResult {
	firstTag: unknown;
	id: number;
	name: string;
	secondTag: unknown;
	thirdTag: unknown;
}

export interface IJsonArrayExtractQuery {
	params: JsonArrayExtractParams;
	result: IJsonArrayExtractResult;
}

export type JsonArrayContainsParams = [];

export interface IJsonArrayContainsResult {
	hasDatabase: unknown;
	hasMysql: unknown;
	id: number;
	name: string;
	tags: unknown;
}

export interface IJsonArrayContainsQuery {
	params: JsonArrayContainsParams;
	result: IJsonArrayContainsResult;
}

export type JsonArrayMembershipParams = [];

export interface IJsonArrayMembershipResult {
	hasMysqlTag: unknown;
	hasTutorialTag: unknown;
	id: number;
	name: string;
}

export interface IJsonArrayMembershipQuery {
	params: JsonArrayMembershipParams;
	result: IJsonArrayMembershipResult;
}

export type JsonNestedArrayAccessParams = [];

export interface IJsonNestedArrayAccessResult {
	firstItemName: unknown;
	firstItemPrice: unknown;
	id: number;
	name: string;
	secondItemName: unknown;
	secondItemQuantity: unknown;
}

export interface IJsonNestedArrayAccessQuery {
	params: JsonNestedArrayAccessParams;
	result: IJsonNestedArrayAccessResult;
}

export type JsonDeepNestedArrayParams = [];

export interface IJsonDeepNestedArrayResult {
	firstAchievement: unknown;
	firstInventoryItem: unknown;
	firstItemRarity: unknown;
	id: number;
	name: string;
	secondInventoryItem: unknown;
}

export interface IJsonDeepNestedArrayQuery {
	params: JsonDeepNestedArrayParams;
	result: IJsonDeepNestedArrayResult;
}

export type JsonArrayBuildParams = [];

export interface IJsonArrayBuildResult {
	firstTwoTags: unknown;
	id: number;
	name: string;
}

export interface IJsonArrayBuildQuery {
	params: JsonArrayBuildParams;
	result: IJsonArrayBuildResult;
}

export type JsonArrayAppendParams = [];

export interface IJsonArrayAppendResult {
	id: number;
	name: string;
	originalTags: unknown;
	tagsWithNew: unknown;
}

export interface IJsonArrayAppendQuery {
	params: JsonArrayAppendParams;
	result: IJsonArrayAppendResult;
}

export type JsonArrayInsertParams = [];

export interface IJsonArrayInsertResult {
	id: number;
	name: string;
	originalTags: unknown;
	tagsWithInsert: unknown;
}

export interface IJsonArrayInsertQuery {
	params: JsonArrayInsertParams;
	result: IJsonArrayInsertResult;
}


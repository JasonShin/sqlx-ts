export type JsonArrayLengthParams = [];

export interface IJsonArrayLengthResult {
	id: number;
	itemsCount: any;
	name: string;
	tagsCount: any;
}

export interface IJsonArrayLengthQuery {
	params: JsonArrayLengthParams;
	result: IJsonArrayLengthResult;
}

export type JsonArrayExtractParams = [];

export interface IJsonArrayExtractResult {
	firstTag: any;
	id: number;
	name: string;
	secondTag: any;
	thirdTag: any;
}

export interface IJsonArrayExtractQuery {
	params: JsonArrayExtractParams;
	result: IJsonArrayExtractResult;
}

export type JsonArrayContainsParams = [];

export interface IJsonArrayContainsResult {
	hasDatabase: any;
	hasMysql: any;
	id: number;
	name: string;
	tags: any;
}

export interface IJsonArrayContainsQuery {
	params: JsonArrayContainsParams;
	result: IJsonArrayContainsResult;
}

export type JsonArrayMembershipParams = [];

export interface IJsonArrayMembershipResult {
	hasMysqlTag: any;
	hasTutorialTag: any;
	id: number;
	name: string;
}

export interface IJsonArrayMembershipQuery {
	params: JsonArrayMembershipParams;
	result: IJsonArrayMembershipResult;
}

export type JsonNestedArrayAccessParams = [];

export interface IJsonNestedArrayAccessResult {
	firstItemName: any;
	firstItemPrice: any;
	id: number;
	name: string;
	secondItemName: any;
	secondItemQuantity: any;
}

export interface IJsonNestedArrayAccessQuery {
	params: JsonNestedArrayAccessParams;
	result: IJsonNestedArrayAccessResult;
}

export type JsonDeepNestedArrayParams = [];

export interface IJsonDeepNestedArrayResult {
	firstAchievement: any;
	firstInventoryItem: any;
	firstItemRarity: any;
	id: number;
	name: string;
	secondInventoryItem: any;
}

export interface IJsonDeepNestedArrayQuery {
	params: JsonDeepNestedArrayParams;
	result: IJsonDeepNestedArrayResult;
}

export type JsonArrayBuildParams = [];

export interface IJsonArrayBuildResult {
	firstTwoTags: any;
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
	originalTags: any;
	tagsWithNew: any;
}

export interface IJsonArrayAppendQuery {
	params: JsonArrayAppendParams;
	result: IJsonArrayAppendResult;
}

export type JsonArrayInsertParams = [];

export interface IJsonArrayInsertResult {
	id: number;
	name: string;
	originalTags: any;
	tagsWithInsert: any;
}

export interface IJsonArrayInsertQuery {
	params: JsonArrayInsertParams;
	result: IJsonArrayInsertResult;
}

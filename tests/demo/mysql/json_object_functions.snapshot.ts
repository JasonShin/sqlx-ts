export type JsonObjectKeysParams = [];

export interface IJsonObjectKeysResult {
	id: number;
	name: string;
	objectKeys: any;
}

export interface IJsonObjectKeysQuery {
	params: JsonObjectKeysParams;
	result: IJsonObjectKeysResult;
}

export type JsonObjectKeysPathParams = [];

export interface IJsonObjectKeysPathResult {
	addressKeys: any;
	id: number;
	name: string;
}

export interface IJsonObjectKeysPathQuery {
	params: JsonObjectKeysPathParams;
	result: IJsonObjectKeysPathResult;
}

export type JsonTypeofParams = [];

export interface IJsonTypeofResult {
	activeType: any;
	ageType: any;
	id: number;
	itemsType: any;
	name: string;
	tagsType: any;
	usernameType: any;
}

export interface IJsonTypeofQuery {
	params: JsonTypeofParams;
	result: IJsonTypeofResult;
}

export type JsonContainsParams = [];

export interface IJsonContainsResult {
	hasSpecificUsername: any;
	id: number;
	isActive: any;
	name: string;
}

export interface IJsonContainsQuery {
	params: JsonContainsParams;
	result: IJsonContainsResult;
}

export type JsonContainsPathParams = [];

export interface IJsonContainsPathResult {
	hasAddress: any;
	hasBoth: any;
	hasNonexistent: any;
	hasUsername: any;
	id: number;
	name: string;
}

export interface IJsonContainsPathQuery {
	params: JsonContainsPathParams;
	result: IJsonContainsPathResult;
}

export type JsonObjectBuildParams = [];

export interface IJsonObjectBuildResult {
	id: number;
	name: string;
	userSummary: any;
}

export interface IJsonObjectBuildQuery {
	params: JsonObjectBuildParams;
	result: IJsonObjectBuildResult;
}

export type JsonSetParams = [];

export interface IJsonSetResult {
	id: number;
	name: string;
	originalData: object;
	updatedAge: any;
	updatedCity: any;
}

export interface IJsonSetQuery {
	params: JsonSetParams;
	result: IJsonSetResult;
}

export type JsonInsertParams = [];

export interface IJsonInsertResult {
	id: number;
	name: string;
	originalData: object;
	withPhone: any;
}

export interface IJsonInsertQuery {
	params: JsonInsertParams;
	result: IJsonInsertResult;
}

export type JsonReplaceParams = [];

export interface IJsonReplaceResult {
	id: number;
	name: string;
	originalData: object;
	withNewUsername: any;
}

export interface IJsonReplaceQuery {
	params: JsonReplaceParams;
	result: IJsonReplaceResult;
}

export type JsonRemoveParams = [];

export interface IJsonRemoveResult {
	id: number;
	name: string;
	originalData: object;
	withoutAge: any;
}

export interface IJsonRemoveQuery {
	params: JsonRemoveParams;
	result: IJsonRemoveResult;
}

export type JsonMergePatchParams = [];

export interface IJsonMergePatchResult {
	id: number;
	mergedData: any;
	name: string;
	originalData: object;
}

export interface IJsonMergePatchQuery {
	params: JsonMergePatchParams;
	result: IJsonMergePatchResult;
}

export type JsonMergePreserveParams = [];

export interface IJsonMergePreserveResult {
	id: number;
	mergedData: any;
	name: string;
	originalData: object;
}

export interface IJsonMergePreserveQuery {
	params: JsonMergePreserveParams;
	result: IJsonMergePreserveResult;
}

export type JsonSearchParams = [];

export interface IJsonSearchResult {
	emailPath: any;
	id: number;
	name: string;
	usernamePath: any;
}

export interface IJsonSearchQuery {
	params: JsonSearchParams;
	result: IJsonSearchResult;
}

export type JsonDepthParams = [];

export interface IJsonDepthResult {
	dataDepth: any;
	id: number;
	name: string;
}

export interface IJsonDepthQuery {
	params: JsonDepthParams;
	result: IJsonDepthResult;
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


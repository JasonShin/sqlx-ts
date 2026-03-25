export type JsonObjectKeysParams = [];

export interface IJsonObjectKeysResult {
	id: number;
	name: string;
	objectKeys: unknown;
}

export interface IJsonObjectKeysQuery {
	params: JsonObjectKeysParams;
	result: IJsonObjectKeysResult;
}

export type JsonObjectKeysPathParams = [];

export interface IJsonObjectKeysPathResult {
	addressKeys: unknown;
	id: number;
	name: string;
}

export interface IJsonObjectKeysPathQuery {
	params: JsonObjectKeysPathParams;
	result: IJsonObjectKeysPathResult;
}

export type JsonTypeofParams = [];

export interface IJsonTypeofResult {
	activeType: unknown;
	ageType: unknown;
	id: number;
	itemsType: unknown;
	name: string;
	tagsType: unknown;
	usernameType: unknown;
}

export interface IJsonTypeofQuery {
	params: JsonTypeofParams;
	result: IJsonTypeofResult;
}

export type JsonContainsParams = [];

export interface IJsonContainsResult {
	hasSpecificUsername: unknown;
	id: number;
	isActive: unknown;
	name: string;
}

export interface IJsonContainsQuery {
	params: JsonContainsParams;
	result: IJsonContainsResult;
}

export type JsonContainsPathParams = [];

export interface IJsonContainsPathResult {
	hasAddress: unknown;
	hasBoth: unknown;
	hasNonexistent: unknown;
	hasUsername: unknown;
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
	userSummary: { id: number; name: string; username: any; email: any };
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
	updatedAge: unknown;
	updatedCity: unknown;
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
	withPhone: unknown;
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
	withNewUsername: unknown;
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
	withoutAge: unknown;
}

export interface IJsonRemoveQuery {
	params: JsonRemoveParams;
	result: IJsonRemoveResult;
}

export type JsonMergePatchParams = [];

export interface IJsonMergePatchResult {
	id: number;
	mergedData: unknown;
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
	mergedData: unknown;
	name: string;
	originalData: object;
}

export interface IJsonMergePreserveQuery {
	params: JsonMergePreserveParams;
	result: IJsonMergePreserveResult;
}

export type JsonSearchParams = [];

export interface IJsonSearchResult {
	emailPath: unknown;
	id: number;
	name: string;
	usernamePath: unknown;
}

export interface IJsonSearchQuery {
	params: JsonSearchParams;
	result: IJsonSearchResult;
}

export type JsonDepthParams = [];

export interface IJsonDepthResult {
	dataDepth: unknown;
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
	isValidJson: unknown;
	name: string;
}

export interface IJsonValidQuery {
	params: JsonValidParams;
	result: IJsonValidResult;
}


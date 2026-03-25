export type JsonbObjectKeysParams = [];

export interface IJsonbObjectKeysResult {
	jsonTestDataId: number;
	jsonTestDataName: string;
	objectKey: unknown;
}

export interface IJsonbObjectKeysQuery {
	params: JsonbObjectKeysParams;
	result: IJsonbObjectKeysResult;
}

export type JsonbTypeofParams = [];

export interface IJsonbTypeofResult {
	activeType: unknown;
	ageType: unknown;
	itemsType: unknown;
	jsonTestDataId: number;
	jsonTestDataName: string;
	tagsType: unknown;
	usernameType: unknown;
}

export interface IJsonbTypeofQuery {
	params: JsonbTypeofParams;
	result: IJsonbTypeofResult;
}

export type JsonbStripNullsParams = [];

export interface IJsonbStripNullsResult {
	jsonTestDataId: number;
	jsonTestDataName: string;
	reviewWithNulls: number;
	reviewWithoutNulls: unknown;
}

export interface IJsonbStripNullsQuery {
	params: JsonbStripNullsParams;
	result: IJsonbStripNullsResult;
}

export type JsonbKeyExistsParams = [];

export interface IJsonbKeyExistsResult {
	hasAddress: string;
	hasNonexistent: string;
	hasUsername: string;
	jsonTestDataId: number;
	jsonTestDataName: string;
}

export interface IJsonbKeyExistsQuery {
	params: JsonbKeyExistsParams;
	result: IJsonbKeyExistsResult;
}

export type JsonbAnyKeyExistsParams = [];

export interface IJsonbAnyKeyExistsResult {
	hasAnyContact: unknown;
	jsonTestDataId: number;
	jsonTestDataName: string;
}

export interface IJsonbAnyKeyExistsQuery {
	params: JsonbAnyKeyExistsParams;
	result: IJsonbAnyKeyExistsResult;
}

export type JsonbAllKeysExistParams = [];

export interface IJsonbAllKeysExistResult {
	hasAllRequired: unknown;
	hasAllWithPhone: unknown;
	jsonTestDataId: number;
	jsonTestDataName: string;
}

export interface IJsonbAllKeysExistQuery {
	params: JsonbAllKeysExistParams;
	result: IJsonbAllKeysExistResult;
}

export type JsonbContainsParams = [];

export interface IJsonbContainsResult {
	hasSpecificUsername: object;
	isActive: object;
	jsonTestDataId: number;
	jsonTestDataName: string;
}

export interface IJsonbContainsQuery {
	params: JsonbContainsParams;
	result: IJsonbContainsResult;
}

export type JsonbContainedByParams = [];

export interface IJsonbContainedByResult {
	jsonTestDataId: number;
	jsonTestDataName: string;
	subsetInData: object;
	usernameInData: object;
}

export interface IJsonbContainedByQuery {
	params: JsonbContainedByParams;
	result: IJsonbContainedByResult;
}

export type JsonbSetParams = [];

export interface IJsonbSetResult {
	jsonTestDataId: number;
	jsonTestDataName: string;
	originalData: object;
	updatedAge: unknown;
	updatedCity: unknown;
}

export interface IJsonbSetQuery {
	params: JsonbSetParams;
	result: IJsonbSetResult;
}

export type JsonbInsertParams = [];

export interface IJsonbInsertResult {
	jsonTestDataId: number;
	jsonTestDataName: string;
	originalData: object;
	withPhone: unknown;
}

export interface IJsonbInsertQuery {
	params: JsonbInsertParams;
	result: IJsonbInsertResult;
}


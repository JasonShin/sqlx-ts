export type JsonbObjectKeysParams = [];

export interface IJsonbObjectKeysResult {
	jsonTestDataId: number;
	jsonTestDataName: string;
	objectKey: any;
}

export interface IJsonbObjectKeysQuery {
	params: JsonbObjectKeysParams;
	result: IJsonbObjectKeysResult;
}

export type JsonbTypeofParams = [];

export interface IJsonbTypeofResult {
	activeType: any;
	ageType: any;
	itemsType: any;
	jsonTestDataId: number;
	jsonTestDataName: string;
	tagsType: any;
	usernameType: any;
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
	reviewWithoutNulls: any;
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
	hasAnyContact: any;
	jsonTestDataId: number;
	jsonTestDataName: string;
}

export interface IJsonbAnyKeyExistsQuery {
	params: JsonbAnyKeyExistsParams;
	result: IJsonbAnyKeyExistsResult;
}

export type JsonbAllKeysExistParams = [];

export interface IJsonbAllKeysExistResult {
	hasAllRequired: any;
	hasAllWithPhone: any;
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
	updatedAge: any;
	updatedCity: any;
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
	withPhone: any;
}

export interface IJsonbInsertQuery {
	params: JsonbInsertParams;
	result: IJsonbInsertResult;
}

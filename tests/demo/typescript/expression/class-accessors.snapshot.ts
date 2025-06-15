export type GetterQueryParams = [];

export interface IGetterQueryResult {
	id: number;
}

export interface IGetterQueryQuery {
	params: GetterQueryParams;
	result: IGetterQueryResult;
}

export type SetterQueryParams = [];

export interface ISetterQueryResult {
	id: number;
}

export interface ISetterQueryQuery {
	params: SetterQueryParams;
	result: ISetterQueryResult;
}

export type DefaultMethodQueryParams = [];

export interface IDefaultMethodQueryResult {
	id: number;
}

export interface IDefaultMethodQueryQuery {
	params: DefaultMethodQueryParams;
	result: IDefaultMethodQueryResult;
}

export type PrivateMethodQueryParams = [];

export interface IPrivateMethodQueryResult {
	id: number;
}

export interface IPrivateMethodQueryQuery {
	params: PrivateMethodQueryParams;
	result: IPrivateMethodQueryResult;
}

export type PublicMethodQueryParams = [];

export interface IPublicMethodQueryResult {
	id: number;
}

export interface IPublicMethodQueryQuery {
	params: PublicMethodQueryParams;
	result: IPublicMethodQueryResult;
}

export type TestClassPropertyQueryParams = [];

export interface ITestClassPropertyQueryResult {
	id: number;
}

export interface ITestClassPropertyQueryQuery {
	params: TestClassPropertyQueryParams;
	result: ITestClassPropertyQueryResult;
}

export type Sql2Params = [];

export interface ISql2Result {
	id: number;
}

export interface ISql2Query {
	params: Sql2Params;
	result: ISql2Result;
}

export type TestClassConstructorQueryParams = [];

export interface ITestClassConstructorQueryResult {
	id: number;
}

export interface ITestClassConstructorQueryQuery {
	params: TestClassConstructorQueryParams;
	result: ITestClassConstructorQueryResult;
}

export type SomeConstructorQueryParams = [];

export interface ISomeConstructorQueryResult {
	id: number;
}

export interface ISomeConstructorQueryQuery {
	params: SomeConstructorQueryParams;
	result: ISomeConstructorQueryResult;
}

export type TestClassMethodQueryParams = [];

export interface ITestClassMethodQueryResult {
	id: number;
}

export interface ITestClassMethodQueryQuery {
	params: TestClassMethodQueryParams;
	result: ITestClassMethodQueryResult;
}

export type SomeMethodQueryParams = [];

export interface ISomeMethodQueryResult {
	id: number;
}

export interface ISomeMethodQueryQuery {
	params: SomeMethodQueryParams;
	result: ISomeMethodQueryResult;
}

export type TestChildClassConstructorQueryParams = [];

export interface ITestChildClassConstructorQueryResult {
	id: number;
}

export interface ITestChildClassConstructorQueryQuery {
	params: TestChildClassConstructorQueryParams;
	result: ITestChildClassConstructorQueryResult;
}

export type PrivAutoAccessorPropParams = [];

export interface IPrivAutoAccessorPropResult {
	flavor_text: string | null;
	id: number;
	inventory_id: number | null;
	name: string;
	rarity: string | null;
}

export interface IPrivAutoAccessorPropQuery {
	params: PrivAutoAccessorPropParams;
	result: IPrivAutoAccessorPropResult;
}

export type AutoAccessorPropParams = [];

export interface IAutoAccessorPropResult {
	flavor_text: string | null;
	id: number;
	inventory_id: number | null;
	name: string;
	rarity: string | null;
}

export interface IAutoAccessorPropQuery {
	params: AutoAccessorPropParams;
	result: IAutoAccessorPropResult;
}


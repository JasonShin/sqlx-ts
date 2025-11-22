export type BasicAsConstParams = [];

export interface IBasicAsConstResult {
	id: number;
	name: string;
}

export interface IBasicAsConstQuery {
	params: BasicAsConstParams;
	result: IBasicAsConstResult;
}

export type FunctionReturningAsConstParams = [string | null];

export interface IFunctionReturningAsConstResult {
	id: number;
	name: string;
}

export interface IFunctionReturningAsConstQuery {
	params: FunctionReturningAsConstParams;
	result: IFunctionReturningAsConstResult;
}

export type NestedAsConstParams = [number];

export interface INestedAsConstResult {
	id: number;
	name: string;
}

export interface INestedAsConstQuery {
	params: NestedAsConstParams;
	result: INestedAsConstResult;
}

export type AsConstWithTypeAssertionParams = [];

export interface IAsConstWithTypeAssertionResult {
	id: number;
	name: string;
}

export interface IAsConstWithTypeAssertionQuery {
	params: AsConstWithTypeAssertionParams;
	result: IAsConstWithTypeAssertionResult;
}


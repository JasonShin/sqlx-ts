export type ConcatFunctionParams = [];

export interface IConcatFunctionResult {
	displayName: string;
	id: number;
}

export interface IConcatFunctionQuery {
	params: ConcatFunctionParams;
	result: IConcatFunctionResult;
}

export type UpperLowerFunctionsParams = [];

export interface IUpperLowerFunctionsResult {
	id: number;
	nameLower: string;
	nameUpper: string;
}

export interface IUpperLowerFunctionsQuery {
	params: UpperLowerFunctionsParams;
	result: IUpperLowerFunctionsResult;
}

export type LengthFunctionParams = [];

export interface ILengthFunctionResult {
	id: number;
	name: string;
	nameLength: string;
}

export interface ILengthFunctionQuery {
	params: LengthFunctionParams;
	result: ILengthFunctionResult;
}

export type SubstringFunctionParams = [];

export interface ISubstringFunctionResult {
	id: number;
	name: string;
	namePrefix: string;
}

export interface ISubstringFunctionQuery {
	params: SubstringFunctionParams;
	result: ISubstringFunctionResult;
}

export type TrimFunctionsParams = [];

export interface ITrimFunctionsResult {
	id: number;
	nameLtrim: string;
	nameRtrim: string;
	nameTrimmed: string;
}

export interface ITrimFunctionsQuery {
	params: TrimFunctionsParams;
	result: ITrimFunctionsResult;
}

export type ReplaceFunctionParams = [];

export interface IReplaceFunctionResult {
	id: number;
	name: string;
	nameReplaced: string;
}

export interface IReplaceFunctionQuery {
	params: ReplaceFunctionParams;
	result: IReplaceFunctionResult;
}


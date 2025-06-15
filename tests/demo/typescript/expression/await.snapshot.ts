export type AsyncPlainFunctionParams = [];

export interface IAsyncPlainFunctionResult {
	id: number;
}

export interface IAsyncPlainFunctionQuery {
	params: AsyncPlainFunctionParams;
	result: IAsyncPlainFunctionResult;
}

export type AsyncLambdaAwaitedParams = [];

export interface IAsyncLambdaAwaitedResult {
	id: number;
}

export interface IAsyncLambdaAwaitedQuery {
	params: AsyncLambdaAwaitedParams;
	result: IAsyncLambdaAwaitedResult;
}

export type IifLambdaParams = [];

export interface IIifLambdaResult {
	id: number;
}

export interface IIifLambdaQuery {
	params: IifLambdaParams;
	result: IIifLambdaResult;
}


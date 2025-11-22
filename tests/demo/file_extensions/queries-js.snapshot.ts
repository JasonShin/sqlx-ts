export type JsQueryParams = [string | null];

export interface IJsQueryResult {
	id: number;
	name: string;
}

export interface IJsQueryQuery {
	params: JsQueryParams;
	result: IJsQueryResult;
}

export type JsFunctionQueryParams = [number];

export interface IJsFunctionQueryResult {
	id: number;
	name: string;
}

export interface IJsFunctionQueryQuery {
	params: JsFunctionQueryParams;
	result: IJsFunctionQueryResult;
}


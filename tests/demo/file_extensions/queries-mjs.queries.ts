export type MjsQueryParams = [string | null];

export interface IMjsQueryResult {
	id: number;
	name: string;
}

export interface IMjsQueryQuery {
	params: MjsQueryParams;
	result: IMjsQueryResult;
}

export type MjsFunctionQueryParams = [number];

export interface IMjsFunctionQueryResult {
	id: number;
	name: string;
}

export interface IMjsFunctionQueryQuery {
	params: MjsFunctionQueryParams;
	result: IMjsFunctionQueryResult;
}

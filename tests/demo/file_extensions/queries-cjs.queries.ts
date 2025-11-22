export type CjsQueryParams = [string | null];

export interface ICjsQueryResult {
	id: number;
	name: string;
}

export interface ICjsQueryQuery {
	params: CjsQueryParams;
	result: ICjsQueryResult;
}

export type CjsFunctionQueryParams = [number];

export interface ICjsFunctionQueryResult {
	id: number;
	name: string;
}

export interface ICjsFunctionQueryQuery {
	params: CjsFunctionQueryParams;
	result: ICjsFunctionQueryResult;
}

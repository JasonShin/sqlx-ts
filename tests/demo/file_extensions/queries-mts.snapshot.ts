export type MtsQueryParams = [string | null];

export interface IMtsQueryResult {
	id: number;
	name: string;
}

export interface IMtsQueryQuery {
	params: MtsQueryParams;
	result: IMtsQueryResult;
}

export type MtsFunctionQueryParams = [number];

export interface IMtsFunctionQueryResult {
	id: number;
	name: string;
}

export interface IMtsFunctionQueryQuery {
	params: MtsFunctionQueryParams;
	result: IMtsFunctionQueryResult;
}


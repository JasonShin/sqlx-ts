export type CtsQueryParams = [string | null];

export interface ICtsQueryResult {
	id: number;
	name: string;
}

export interface ICtsQueryQuery {
	params: CtsQueryParams;
	result: ICtsQueryResult;
}

export type CtsFunctionQueryParams = [number];

export interface ICtsFunctionQueryResult {
	id: number;
	name: string;
}

export interface ICtsFunctionQueryQuery {
	params: CtsFunctionQueryParams;
	result: ICtsFunctionQueryResult;
}


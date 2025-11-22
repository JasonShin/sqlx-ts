export type UpdateQueryParams = [string, number];

export interface IUpdateQueryResult {
	id: number;
	name: string;
}

export interface IUpdateQueryQuery {
	params: UpdateQueryParams;
	result: IUpdateQueryResult;
}

export type DeleteQueryParams = [number];

export interface IDeleteQueryResult {
	id: number;
}

export interface IDeleteQueryQuery {
	params: DeleteQueryParams;
	result: IDeleteQueryResult;
}
